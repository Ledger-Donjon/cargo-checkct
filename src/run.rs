// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{
    env, fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Mutex,
    thread,
    time::Duration,
};

use anyhow::{Context, Result, bail};
use clap::ValueEnum;
use goblin::elf::{
    Elf,
    section_header::{SHT_NOBITS, SHT_NOTE},
};
use which::which;

/// Verification status, ordered from best to worst.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Secure,
    Unknown,
    Error,
    Insecure,
}

impl Status {
    fn as_str(self) -> &'static str {
        match self {
            Status::Secure => "secure",
            Status::Unknown => "unknown",
            Status::Error => "error",
            Status::Insecure => "insecure",
        }
    }
}

/// The constant-time checks that binsec can perform (see `binsec -checkct-help`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Check {
    /// Secret-dependent branches
    ControlFlow,
    /// Secret-dependent memory accesses
    MemoryAccess,
    /// Multiplications with secret operands (experimental)
    Multiplication,
    /// Divisions with a secret dividend (experimental)
    Dividend,
    /// Divisions with a secret divisor (experimental)
    Divisor,
}

pub struct Options {
    /// binsec timeout, for each entrypoint
    pub timeout: Duration,
    pub checks: Vec<Check>,
    /// Keep exploring after the first leak, to report all the leaky instructions
    pub all_leaks: bool,
    /// Number of binsec instances to run in parallel
    pub jobs: usize,
}

/// The result of the verification of one entrypoint, for one target.
pub struct Report {
    pub driver: String,
    pub target: String,
    pub entrypoint: String,
    pub status: Status,
    /// Leaky instructions reported by binsec, or error messages
    pub details: Vec<String>,
}

struct Abi {
    lr: &'static str,
    ret: &'static str,
    thumb: &'static str,
    size: usize,
    return_res: &'static str,
    /// Additional script statements
    extra: &'static str,
    isa: Option<&'static str>,
}

impl Abi {
    fn for_target(target: &str) -> Result<Self> {
        let arch = target.split('-').next().unwrap_or_default();
        Ok(if arch.starts_with("thumb") {
            Abi {
                lr: "lr",
                ret: "0x8badf00d ^ 1",
                thumb: " ^1",
                size: 32,
                // binsec's `return` jumps to lr without clearing the thumb bit
                return_res: "r0 := res\n    jump at lr & 0xfffffffe",
                extra: "",
                isa: Some("armv7:thumb"),
            }
        } else if arch.starts_with("riscv32") {
            Abi {
                lr: "ra",
                // `ret` clears the least significant bit of the return address
                ret: "0x8badf00d ^ 1",
                thumb: "",
                size: 32,
                return_res: "return res",
                extra: "",
                isa: None,
            }
        } else if arch == "x86_64" {
            Abi {
                lr: "@[rsp, 8]",
                ret: "0xffffffff8badf00d",
                thumb: "",
                size: 64,
                return_res: "return res",
                // The direction flag is cleared on function entry (otherwise binsec explores
                // `rep` string instructions, e.g. in memset, in both directions).
                // binsec does not support cpuid, so model a CPU without optional features
                // (e.g. for runtime feature detection with the cpufeatures crate).
                extra: "DF := 0\n\
                        replace opcode 0f a2 by\n    rax := 0\n    rbx := 0\n    rcx := 0\n    rdx := 0\nend",
                isa: None,
            }
        } else {
            bail!(
                "unsupported target `{target}`: only thumb*, riscv32* and x86_64 targets are supported \
                 (make sure that `build.target` is set in checkct/.cargo/config.toml)"
            )
        })
    }
}

/// A driver binary, built for a given target.
struct Binary {
    driver: String,
    target: String,
    path: PathBuf,
}

/// A binsec analysis to run.
struct Job {
    driver: String,
    target: String,
    entrypoint: String,
    command: Command,
}

struct Entrypoint {
    /// The (mangled) symbol of the entrypoint function
    symbol: String,
    /// The demangled path of the entrypoint function
    name: String,
}

fn find_checkct_entrypoints(elf: &Elf, binary: &[u8]) -> Result<Vec<Entrypoint>> {
    let mut checkct_entrypoints = Vec::new();
    for sym in elf.syms.iter() {
        // Find checkct entrypoint descriptors by looking for symbols whose name contains __checkct_entrypoint_descriptor__
        let Some(symbol_name) = elf.strtab.get_at(sym.st_name) else {
            continue;
        };
        if !symbol_name.contains("__checkct_entrypoint_descriptor__") {
            continue;
        }

        // The descriptor ends with the address of the entrypoint function
        let section_header = elf
            .section_headers
            .get(sym.st_shndx)
            .with_context(|| format!("Invalid section index for symbol {symbol_name}"))?;
        let pointer_size = if elf.is_64 { 8 } else { 4 };
        if sym.st_size == pointer_size && section_header.sh_type == SHT_NOTE {
            bail!(
                "{symbol_name} was generated by an outdated version of checkct_macros, which binsec >= 0.11 \
                 cannot load (see the migration instructions in the cargo-checkct README)"
            );
        }
        let bytes = sym
            .st_value
            .checked_sub(section_header.sh_addr)
            .map(|offset| offset + section_header.sh_offset + sym.st_size)
            .and_then(|end| Some(end.checked_sub(pointer_size)?..end))
            .and_then(|range| binary.get(range.start as usize..range.end as usize))
            .with_context(|| format!("Symbol {symbol_name} is out of the bounds of its section"))?;
        let entry_addr = match (bytes.len(), elf.little_endian) {
            (4, true) => u32::from_le_bytes(bytes.try_into()?).into(),
            (4, false) => u32::from_be_bytes(bytes.try_into()?).into(),
            (_, true) => u64::from_le_bytes(bytes.try_into()?),
            (_, false) => u64::from_be_bytes(bytes.try_into()?),
        };

        let entry_symbol = elf
            .syms
            .iter()
            .find(|s| s.st_value == entry_addr && s.is_function())
            .and_then(|s| elf.strtab.get_at(s.st_name))
            .with_context(|| {
                format!("Failed to find the entrypoint function referenced by {symbol_name} at {entry_addr:#x}")
            })?;
        checkct_entrypoints.push(Entrypoint {
            symbol: entry_symbol.to_owned(),
            name: format!("{:#}", rustc_demangle::demangle(entry_symbol)),
        });
    }

    Ok(checkct_entrypoints)
}

/// Workspaces created by older versions of cargo-checkct do not configure a linker for the
/// `x86_64-unknown-linux-gnu` target, and rely on a gcc cross-compiler instead.
fn legacy_x86_64_linker(dir: &Path) -> Result<Option<&'static str>> {
    if env::var_os("CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER").is_some() {
        return Ok(None);
    }

    let config_path = dir.join(".cargo").join("config.toml");
    if config_path.exists() {
        let config = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {config_path:?}"))?
            .parse::<toml::Table>()
            .with_context(|| format!("Failed to parse {config_path:?}"))?;
        let target = config
            .get("target")
            .and_then(|target| target.get("x86_64-unknown-linux-gnu"));
        let linker_in_rustflags = target
            .and_then(|target| target.get("rustflags"))
            .and_then(|rustflags| rustflags.as_array())
            .is_some_and(|rustflags| {
                rustflags
                    .iter()
                    .any(|flag| flag.as_str().is_some_and(|flag| flag.contains("linker=")))
            });
        if linker_in_rustflags || target.and_then(|target| target.get("linker")).is_some() {
            return Ok(None);
        }
    }

    Ok(if cfg!(target_os = "macos") {
        Some("x86_64-unknown-linux-gnu-gcc")
    } else if cfg!(target_os = "linux") {
        Some("x86_64-linux-gnu-gcc")
    } else {
        None
    })
}

/// Build the drivers of the checkct workspace at `dir`, and return the resulting binaries.
fn build_drivers(dir: &Path) -> Result<Vec<Binary>> {
    let cargo_path = which("cargo").context("Failed to find cargo")?;
    let mut cmd = Command::new(cargo_path);
    cmd.current_dir(dir)
        .arg("build")
        .arg("--release")
        .arg("--message-format=json-render-diagnostics")
        .stdout(Stdio::piped())
        // Let the rust-toolchain.toml file of the checkct workspace select the toolchain,
        // even when running as `cargo checkct` (rustup then sets RUSTUP_TOOLCHAIN)
        .env_remove("RUSTUP_TOOLCHAIN");

    // These would override the rustflags of the checkct workspace configuration
    for var in ["RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS"] {
        if env::var_os(var).is_some() {
            println!(
                "note: ignoring {var}, set rustflags in {:?} instead",
                dir.join(".cargo").join("config.toml")
            );
            cmd.env_remove(var);
        }
    }

    // Enforce -fno-stack-protector since we build with no_std
    let mut cflags = env::var("CFLAGS").unwrap_or_default();
    cflags.push_str(" -fno-stack-protector");
    cmd.env("CFLAGS", cflags.trim_start());

    if let Some(linker) = legacy_x86_64_linker(dir)? {
        cmd.arg("--config").arg(format!(
            "target.x86_64-unknown-linux-gnu.linker=\"{linker}\""
        ));
    }

    let mut child = cmd.spawn().context("Failed to build drivers")?;
    let mut binaries = Vec::new();
    for line in BufReader::new(child.stdout.take().unwrap()).lines() {
        let line = line.context("Failed to read cargo output")?;
        let Ok(message) = serde_json::from_str::<serde_json::Value>(&line) else {
            println!("{line}");
            continue;
        };
        let is_bin = message["target"]["kind"]
            .as_array()
            .is_some_and(|kinds| kinds.iter().any(|kind| kind == "bin"));
        let (Some(executable), Some(name)) = (
            message["executable"].as_str(),
            message["target"]["name"].as_str(),
        ) else {
            continue;
        };
        if message["reason"] != "compiler-artifact" || !is_bin {
            continue;
        }

        // Binaries are located in <target-dir>/<target>/release/
        let path = PathBuf::from(executable);
        let target = path
            .parent()
            .and_then(Path::parent)
            .and_then(Path::file_name)
            .with_context(|| format!("Unexpected binary path {path:?}"))?
            .to_string_lossy()
            .into_owned();
        binaries.push(Binary {
            driver: name.to_owned(),
            target,
            path,
        });
    }

    if !child.wait().context("Failed to build drivers")?.success() {
        bail!("Error while building drivers, see the cargo output above");
    }

    binaries.sort_by(|a, b| (&a.driver, &a.target).cmp(&(&b.driver, &b.target)));
    Ok(binaries)
}

/// Find binsec and check that its version is supported.
fn find_binsec() -> Result<PathBuf> {
    let binsec_path = which("binsec")
        .context("Failed to find binsec - you might need to run `eval $(opam env)` first")?;
    let output = Command::new(&binsec_path)
        .arg("-version")
        .output()
        .context("Failed to run binsec")?;
    let version = String::from_utf8_lossy(&output.stdout);
    match parse_binsec_version(&version) {
        Some(version) if version < (0, 11) => bail!(
            "binsec {}.{} is not supported, please install binsec >= 0.11",
            version.0,
            version.1
        ),
        Some(_) => {}
        None => println!(
            "warning: could not determine the version of binsec ({:?}), binsec >= 0.11 is required",
            version.trim()
        ),
    }
    Ok(binsec_path)
}

fn parse_binsec_version(output: &str) -> Option<(u32, u32)> {
    let mut version = output
        .trim()
        .strip_prefix("Binsec version ")?
        .split(['.', '-', '+']);
    Some((version.next()?.parse().ok()?, version.next()?.parse().ok()?))
}

/// Prepare the binsec analyses of all the entrypoints of `binary`.
fn prepare_jobs(binary: &Binary, binsec_path: &Path, options: &Options) -> Result<Vec<Job>> {
    let abi = Abi::for_target(&binary.target)?;
    let data =
        fs::read(&binary.path).with_context(|| format!("Failed to read {:?}", binary.path))?;
    let elf = Elf::parse(&data).with_context(|| format!("Failed to parse {:?}", binary.path))?;

    // Load the sections of the memory image that have some content in the binary
    let sections = elf
        .section_headers
        .iter()
        .filter(|h| h.is_alloc() && h.sh_type != SHT_NOBITS)
        .filter_map(|h| elf.shdr_strtab.get_at(h.sh_name))
        .filter(|n| !n.is_empty() && ![".note.gnu.build-id", ".note.checkct"].contains(n))
        .collect::<Vec<_>>()
        .join(", ");

    // Panics end in __checkct_panic, where the analysis of a path must stop
    let has_panic_function = elf
        .syms
        .iter()
        .any(|sym| elf.strtab.get_at(sym.st_name) == Some("__checkct_panic"));
    let halt_at_panic = if has_panic_function {
        format!("halt at <__checkct_panic>{}", abi.thumb)
    } else {
        println!(
            "warning: {:?} does not define __checkct_panic, so panic paths may not be analysed correctly \
             (see the migration instructions in the cargo-checkct README)",
            binary.path
        );
        String::new()
    };

    let entrypoints = find_checkct_entrypoints(&elf, &data)
        .with_context(|| format!("Failed to find checkct entrypoints in {:?}", binary.path))?;
    if entrypoints.is_empty() {
        bail!("no #[checkct] entrypoint found in {:?}", binary.path);
    }

    let mut jobs = Vec::new();
    for entrypoint in entrypoints {
        let script_path = binary.path.with_file_name(format!(
            "{}.{}.binsec",
            binary.driver,
            entrypoint.name.replace("::", "__")
        ));
        let script = format!(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/template/driver.binsec"
            )),
            sections = sections,
            entrypoint = entrypoint.symbol,
            lr = abi.lr,
            ret = abi.ret,
            size = abi.size,
            thumb = abi.thumb,
            return_res = abi.return_res,
            halt_at_panic = halt_at_panic,
            extra = abi.extra,
        );
        fs::write(&script_path, script)
            .with_context(|| format!("Failed to write {script_path:?}"))?;

        let mut command = Command::new(binsec_path);
        command
            .arg("-sse")
            .arg("-checkct")
            .arg("-sse-depth")
            .arg("1000000000")
            .arg("-sse-jump-enum")
            .arg("64")
            .arg("-sse-script")
            .arg(&script_path)
            .arg("-sse-timeout")
            .arg(options.timeout.as_secs().to_string())
            .arg("-checkct-leak-info")
            .arg(if options.all_leaks { "instr" } else { "halt" });
        if let Some(isa) = abi.isa {
            command.arg("-isa").arg(isa);
        }
        if !options.checks.is_empty() {
            command.arg("-checkct-features").arg(
                options
                    .checks
                    .iter()
                    .map(|check| check.to_possible_value().unwrap().get_name().to_owned())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        command.arg(&binary.path);

        jobs.push(Job {
            driver: binary.driver.clone(),
            target: binary.target.clone(),
            entrypoint: entrypoint.name,
            command,
        });
    }

    Ok(jobs)
}

/// Run a binsec analysis, and return the corresponding report along with its log.
fn run_job(mut job: Job) -> (Report, String) {
    let mut log = format!(
        "Driver {}, target {}, entrypoint {}:\n  Running: {}\n",
        job.driver,
        job.target,
        job.entrypoint,
        format_command(&job.command)
    );
    let mut report = Report {
        driver: job.driver,
        target: job.target,
        entrypoint: job.entrypoint,
        status: Status::Error,
        details: Vec::new(),
    };

    let output = match job.command.output() {
        Ok(output) => output,
        Err(e) => {
            report.details.push(format!("failed to run binsec: {e}"));
            log.push_str(&format!("  failed to run binsec: {e}\n"));
            return (report, log);
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    for line in stdout.lines() {
        log.push_str(&format!("  {line}\n"));
    }

    if !output.status.success() {
        report.details.push(format!(
            "binsec failed ({}): {}",
            output.status,
            stderr.trim()
        ));
        log.push_str(&format!("  binsec failed ({}):\n{stderr}\n", output.status));
        return (report, log);
    }

    report.status = parse_status(&stdout).unwrap_or_else(|| {
        log.push_str(&format!("  UNEXPECTED binsec output, stderr:\n{stderr}\n"));
        Status::Unknown
    });
    report.details = stdout
        .lines()
        .filter_map(|line| line.split_once("[checkct:result] Instruction "))
        .map(|(_, leak)| leak.to_owned())
        .collect();
    (report, log)
}

fn parse_status(stdout: &str) -> Option<Status> {
    stdout.lines().find_map(|line| {
        let (_, status) = line.split_once("[checkct:result] Program status is : ")?;
        match status.split_whitespace().next()? {
            "secure" => Some(Status::Secure),
            "insecure" => Some(Status::Insecure),
            "unknown" => Some(Status::Unknown),
            _ => None,
        }
    })
}

fn format_command(command: &Command) -> String {
    std::iter::once(command.get_program())
        .chain(command.get_args())
        .map(|arg| arg.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Build the drivers of the checkct workspace at `dir`, and verify each of their
/// entrypoints with binsec.
pub fn run_binsec(dir: &Path, options: &Options) -> Result<Vec<Report>> {
    if !dir.join("Cargo.toml").exists() {
        bail!("No checkct workspace found in {dir:?}, run `cargo-checkct init` first");
    }

    let binsec_path = find_binsec()?;
    let binaries = build_drivers(dir)?;
    if binaries.is_empty() {
        bail!("Error: no driver binaries were built in {dir:?}");
    }

    let mut reports = Vec::new();
    let mut jobs = Vec::new();
    for binary in &binaries {
        match prepare_jobs(binary, &binsec_path, options) {
            Ok(binary_jobs) => jobs.extend(binary_jobs),
            Err(e) => {
                println!("Driver {}, target {}: {e:#}", binary.driver, binary.target);
                reports.push(Report {
                    driver: binary.driver.clone(),
                    target: binary.target.clone(),
                    entrypoint: "-".to_owned(),
                    status: Status::Error,
                    details: vec![format!("{e:#}")],
                })
            }
        }
    }

    // binsec is single-threaded, so run several instances in parallel
    let job_count = jobs.len();
    let jobs = Mutex::new(jobs.into_iter());
    let results = Mutex::new(Vec::with_capacity(job_count));
    thread::scope(|s| {
        for _ in 0..options.jobs.clamp(1, job_count.max(1)) {
            s.spawn(|| {
                loop {
                    let Some(job) = jobs.lock().unwrap().next() else {
                        break;
                    };
                    let (report, log) = run_job(job);
                    // Print the whole log at once, so that the logs of different jobs do not interleave
                    let mut stdout = std::io::stdout().lock();
                    let _ = writeln!(stdout, "{log}  => {}\n", report.status.as_str());
                    results.lock().unwrap().push(report);
                }
            });
        }
    });

    reports.extend(results.into_inner().unwrap());
    reports.sort_by(|a, b| {
        (&a.driver, &a.target, &a.entrypoint).cmp(&(&b.driver, &b.target, &b.entrypoint))
    });
    Ok(reports)
}

/// Overall status of a run.
pub fn overall_status(reports: &[Report]) -> Status {
    reports
        .iter()
        .map(|report| report.status)
        .max()
        .unwrap_or(Status::Secure)
}

/// Print a summary of the reports, and append it to the GitHub Actions job summary if available.
pub fn summarize(reports: &[Report]) -> Result<()> {
    println!("Summary:");
    let widths = reports.iter().fold((6, 6, 10), |(d, t, e), report| {
        (
            d.max(report.driver.len()),
            t.max(report.target.len()),
            e.max(report.entrypoint.len()),
        )
    });
    println!(
        "  {:<dw$}  {:<tw$}  {:<ew$}  status",
        "driver",
        "target",
        "entrypoint",
        dw = widths.0,
        tw = widths.1,
        ew = widths.2
    );
    for report in reports {
        println!(
            "  {:<dw$}  {:<tw$}  {:<ew$}  {}",
            report.driver,
            report.target,
            report.entrypoint,
            report.status.as_str(),
            dw = widths.0,
            tw = widths.1,
            ew = widths.2
        );
        for detail in &report.details {
            println!("      {detail}");
        }
    }

    if let Some(summary_path) = env::var_os("GITHUB_STEP_SUMMARY") {
        let mut summary = String::from(
            "### cargo-checkct\n\n| driver | target | entrypoint | status | details |\n|---|---|---|---|---|\n",
        );
        for report in reports {
            let icon = match report.status {
                Status::Secure => "✅",
                Status::Unknown => "❔",
                Status::Error | Status::Insecure => "❌",
            };
            summary.push_str(&format!(
                "| {} | {} | `{}` | {icon} {} | {} |\n",
                report.driver,
                report.target,
                report.entrypoint,
                report.status.as_str(),
                report
                    .details
                    .join("<br>")
                    .replace('|', "\\|")
                    .replace('\n', " ")
            ));
        }
        fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&summary_path)
            .and_then(|mut file| writeln!(file, "{summary}"))
            .with_context(|| format!("Failed to write the job summary to {summary_path:?}"))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binsec_version() {
        assert_eq!(
            parse_binsec_version("Binsec version 0.11.3\n"),
            Some((0, 11))
        );
        assert_eq!(parse_binsec_version("Binsec version 0.10.0"), Some((0, 10)));
        assert_eq!(parse_binsec_version("Binsec version 1.2"), Some((1, 2)));
        assert_eq!(parse_binsec_version("Binsec version %VERSION%"), None);
        assert_eq!(parse_binsec_version(""), None);
    }

    #[test]
    fn status() {
        let output = "[sse:info] Exploration completed\n\
                      [checkct:result] Program status is : insecure (0.123)\n";
        assert_eq!(parse_status(output), Some(Status::Insecure));
        assert_eq!(
            parse_status("[checkct:result] Program status is : secure (1.000)"),
            Some(Status::Secure)
        );
        assert_eq!(
            parse_status("[checkct:result] Program status is : unknown (600.0)"),
            Some(Status::Unknown)
        );
        assert_eq!(parse_status("[sse:error] something went wrong"), None);
    }

    #[test]
    fn worst_status() {
        let report = |status| Report {
            driver: String::new(),
            target: String::new(),
            entrypoint: String::new(),
            status,
            details: Vec::new(),
        };
        assert_eq!(overall_status(&[]), Status::Secure);
        assert_eq!(
            overall_status(&[report(Status::Secure), report(Status::Unknown)]),
            Status::Unknown
        );
        assert_eq!(
            overall_status(&[report(Status::Insecure), report(Status::Error)]),
            Status::Insecure
        );
    }

    #[test]
    fn abi() {
        assert_eq!(
            Abi::for_target("thumbv7em-none-eabihf").unwrap().isa,
            Some("armv7:thumb")
        );
        assert_eq!(
            Abi::for_target("riscv32imac-unknown-none-elf")
                .unwrap()
                .size,
            32
        );
        assert_eq!(
            Abi::for_target("x86_64-unknown-linux-gnu").unwrap().size,
            64
        );
        assert!(Abi::for_target("riscv64gc-unknown-none-elf").is_err());
        assert!(Abi::for_target("target").is_err());
    }
}
