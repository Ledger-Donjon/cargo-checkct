// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{fs, io::Write, path::Path, time::Duration};

use anyhow::{Context, Result, bail};
use goblin::{Object, container::Endian, elf::Elf};
use which::which;

use crate::common::get_workspace_members;

pub enum Status {
    Secure,
    Insecure,
    Unknown,
}

struct Abi<'a> {
    lr: &'a str,
    ret: &'a str,
    thumb: &'a str,
    size: usize,
}

fn find_checkct_entrypoints<'a>(elf: &'a Elf, binary: &[u8]) -> Result<Vec<&'a str>> {
    let mut checkct_entrypoints = Vec::new();
    for sym in elf.syms.iter() {
        // Find checkct entrypoint descriptors by looking for symbols whose name contains __checkct_entrypoint_descriptor__
        let Some(symbol_name) = elf.strtab.get_at(sym.st_name) else {
            continue;
        };
        if !symbol_name.contains("__checkct_entrypoint_descriptor__") {
            continue;
        }

        let section_header = &elf.section_headers[sym.st_shndx];
        let start = sym.st_value - section_header.sh_addr + section_header.sh_offset;

        let entry_addr = match sym.st_size {
            4 => match elf.header.endianness()? {
                Endian::Little => u32::from_le_bytes(
                    binary[start as usize..(start + sym.st_size) as usize].try_into()?,
                )
                .into(),
                Endian::Big => u32::from_be_bytes(
                    binary[start as usize..(start + sym.st_size) as usize].try_into()?,
                )
                .into(),
            },
            8 => match elf.header.endianness()? {
                Endian::Little => u64::from_le_bytes(
                    binary[start as usize..(start + sym.st_size) as usize].try_into()?,
                ),

                Endian::Big => u64::from_be_bytes(
                    binary[start as usize..(start + sym.st_size) as usize].try_into()?,
                ),
            },
            _ => todo!(),
        };

        let entry_sym = elf
            .syms
            .iter()
            .find(|s| s.st_value == entry_addr && s.is_function())
            .unwrap();
        checkct_entrypoints.push(elf.strtab.get_at(entry_sym.st_name).unwrap());
    }

    Ok(checkct_entrypoints)
}

pub fn run_binsec(dir: &Path, timeout: Duration) -> Result<Status> {
    let cur_dir = std::env::current_dir()?;

    // First we need to actually build the drivers
    std::env::set_current_dir(dir)
        .with_context(|| format!("Failed to set current directory to {dir:?}"))?;
    let cargo_path = which("cargo").context("Failed to find cargo")?;
    let mut cmd = std::process::Command::new(cargo_path);
    cmd.arg("build").arg("--release");

    // We need to change the linker for the x86 cross-compilation
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    cmd.arg("--config")
        .arg("target.x86_64-unknown-linux-gnu.linker=\"x86_64-unknown-linux-gnu-gcc\"");

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    cmd.arg("--config")
        .arg("target.x86_64-unknown-linux-gnu.linker=\"x86_64-linux-gnu-gcc\"");

    let output = cmd.output().context("Failed to build drivers")?;

    if !output.status.success() {
        bail!(
            "Error while building drivers:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    }
    std::env::set_current_dir(cur_dir)?;

    // Next we recover the actual name of the drivers
    let members = get_workspace_members(dir)?;
    if members.is_empty() {
        bail!("Error: found empty [workspace.members] key - no drivers to build.");
    }

    let mut overall_status = Status::Secure;

    // Now for each driver:
    for driver in members {
        println!("Driver {driver}:");
        // First we recover the targets from the config toml
        let config_path = &dir.join(".cargo").join("config.toml");
        let config = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read the config manifest in: {config_path:?}"))?;
        let config_table = config
            .parse::<toml::Table>()
            .with_context(|| format!("Failed to parse the config manifest in: {config_path:?}"))?;
        let build_entry = config_table.get("build").with_context(|| {
            format!("Failed to find the [build] entry of the config manifest in: {config_path:?}")
        })?;
        let toml::Value::Table(build_table) = build_entry else {
            bail!("[build] entry of the config manifest in: {config_path:?} is not a toml::Table");
        };
        let toml::Value::Array(target_list) = build_table.get("target").with_context(|| {
            format!(
                "Failed to find the [build.target] entry of the config manifest in: {config_path:?}"
            )
        })?
        else {
            bail!("")
        };

        // Now we can generate and run the binsec script for each targets
        for target in target_list {
            let toml::Value::String(target) = target else {
                bail!("")
            };

            println!("  target: {target}");

            let abi = match target.split('-').next().unwrap() {
                arch if arch.starts_with("thumb") => Abi {
                    lr: "lr",
                    ret: "0x8badf00d ^ 1",
                    thumb: " ^1",
                    size: 32,
                },
                arch if arch.starts_with("riscv") => Abi {
                    lr: "ra",
                    ret: "0x8badf00d",
                    thumb: "",
                    size: 32,
                },
                arch if arch.starts_with("x86_64") => Abi {
                    lr: "@[rsp, 8]",
                    ret: "0xffffffff8badf00d",
                    thumb: "",
                    size: 64,
                },
                _ => panic!("unexpected target: {target}"),
            };

            let binary = fs::read(
                dir.join("target")
                    .join(target)
                    .join("release")
                    .join(&driver),
            )?;
            let obj = Object::parse(&binary)?;
            let Object::Elf(elf) = obj else {
                bail!("Object format {obj:?} not supported")
            };

            let sections = elf
                .section_headers
                .iter()
                .map(|h| elf.shdr_strtab.get_at(h.sh_name).unwrap())
                .filter(|n| !n.is_empty() && ![".note.gnu.build-id", ".note.checkct"].contains(n))
                .collect::<Vec<_>>()
                .join(", ");

            let checkct_entrypoints = find_checkct_entrypoints(&elf, &binary)?;

            for entrypoint in checkct_entrypoints {
                let mut script = fs::File::create(
                    dir.join("target")
                        .join(target)
                        .join(format!("{driver}.binsec")),
                )?;
                script.write_all(
                    format!(
                        include_str!(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/template/target/driver.binsec"
                        )),
                        sections = sections,
                        entrypoint = entrypoint,
                        lr = abi.lr,
                        ret = abi.ret,
                        size = abi.size,
                        thumb = abi.thumb,
                    )
                    .as_bytes(),
                )?;

                let binsec_path = which("binsec").context(
                    "Failed to find binsec - you might need to run `eval $(opam env)` first",
                )?;
                let mut binsec_cmd = std::process::Command::new(binsec_path);
                binsec_cmd
                    .arg("-sse")
                    .arg("-checkct")
                    .arg("-sse-depth")
                    .arg("1000000000")
                    .arg("-sse-jump-enum")
                    .arg("64")
                    .arg("-sse-script")
                    .arg(format!(
                        "{}",
                        dir.join("target")
                            .join(target)
                            .join(format!("{driver}.binsec"))
                            .to_string_lossy()
                    ))
                    .arg("-sse-timeout")
                    .arg(format!("{}", timeout.as_secs()))
                    .arg("-arm-supported-modes")
                    .arg("thumb")
                    .arg(format!(
                        "{}",
                        dir.join("target")
                            .join(target)
                            .join("release")
                            .join(&driver)
                            .to_string_lossy()
                    ));

                println!("{}", format!("  Running: {binsec_cmd:?}").replace('\"', ""));
                let output = binsec_cmd.output().context("Failed to run binsec")?;

                let driver_status;

                if !output.status.success() {
                    bail!(
                        "Error while running binsec:\nstdout: {}\nstderr: {}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    );
                } else {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if stdout.contains("[checkct:result] Program status is : insecure") {
                        driver_status = Status::Insecure;
                    } else if stdout.contains("[checkct:result] Program status is : secure") {
                        driver_status = Status::Secure;
                    } else {
                        println!(
                            "UNEXPECTED:\nstderr: {}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                        driver_status = Status::Unknown;
                    }
                }

                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));

                match driver_status {
                    Status::Secure => {}
                    // If even one driver is not secure, fail, but still continue
                    // so that the status of the other drivers can be recovered
                    // from the logs.
                    Status::Insecure => {
                        overall_status = Status::Insecure;
                    }
                    Status::Unknown => {
                        if let Status::Secure = overall_status {
                            overall_status = Status::Unknown;
                        }
                    }
                }
            }
        }
    }

    Ok(overall_status)
}
