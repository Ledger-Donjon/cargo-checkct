use std::{fs, io::Write, path::Path, time::Duration};

use anyhow::{bail, Context, Result};
use goblin::Object;

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

pub fn run_binsec(dir: &Path, timeout: Duration) -> Result<Status> {
    let cur_dir = std::env::current_dir()?;

    // First we need to actually build the drivers
    std::env::set_current_dir(dir)?;
    let output = std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output()
        .context("Failed to build drivers")?;

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
    assert!(
        !members.is_empty(),
        "Error: found empty [workspace.members] key - no drivers to build."
    );

    let mut overall_status = Status::Secure;

    // Now for each driver:
    for driver in members {
        println!("Driver {driver}:");
        // First we recover the targets from the config toml
        let path = &dir.join(".cargo").join("config.toml");
        let config = fs::read_to_string(path)
            .with_context(|| format!("Failed to read the config manifest in: {path:?}"))?;
        let config_table = config
            .parse::<toml::Table>()
            .with_context(|| format!("Failed to parse the config manifest in: {path:?}"))?;
        let build_entry = config_table.get("build").with_context(|| {
            format!("Failed to find the [build] entry of the config manifest in: {path:?}")
        })?;
        let toml::Value::Table(build_table) = build_entry else {
            bail!("[build] entry of the config manifest in: {path:?} is not a toml::Table");
        };
        let toml::Value::Array(target_list) = build_table.get("target").with_context(|| {
            format!("Failed to find the [build.target] entry of the config manifest in: {path:?}")
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
            let sections = match obj {
                Object::Elf(elf) => elf
                    .section_headers
                    .iter()
                    .map(|h| elf.shdr_strtab.get_at(h.sh_name).unwrap())
                    .filter(|n| [".text", ".got", ".data", ".rodata"].contains(n))
                    .collect::<Vec<_>>()
                    .join(", "),
                _ => bail!("Object format {obj:?} not supported"),
            };

            let mut script = fs::File::create(
                dir.join("target")
                    .join(target)
                    .join(format!("{driver}.binsec")),
            )?;
            script.write_all(
                format!(
                    r#"load sections {sections} from file
starting from <__checkct>
with concrete stack pointer
{lr} := {ret}
replace <__checkct_private_rand>{thumb} () by
res<{size}> := secret
return res
end
replace <__checkct_public_rand>{thumb} () by
res<{size}> := nondet
return res
end
halt at {ret}
explore all
"#,
                    lr = abi.lr,
                    ret = abi.ret,
                    size = abi.size,
                    thumb = abi.thumb,
                )
                .as_bytes(),
            )?;

            let mut binsec_cmd = std::process::Command::new("binsec");
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

            println!(
                "{}",
                format!("  Running: {:?}", binsec_cmd).replace('\"', "")
            );
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
                    overall_status = Status::Unknown;
                }
            }
        }
    }

    Ok(overall_status)
}
