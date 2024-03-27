use std::{fs, io::Write, path::Path, time::Duration};

use anyhow::{bail, Context, Result};
use cargo_manifest::Manifest;
use goblin::Object;

pub fn run_binsec(dir: &Path, timeout: Duration) -> Result<()> {
    let cur_dir = std::env::current_dir()?;

    // First we need to actually build the drivers
    std::env::set_current_dir(dir)?;
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("cargo build --release")
        .output()
        .context("Failed to build drivers")?;

    if !output.status.success() {
        bail!(
            "Error while building drivers:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    }

    // Next we recover the actual name of the drivers
    let manifest = Manifest::from_path("Cargo.toml")
        .with_context(|| format!("Failed to find the cargo manifest at: {dir:?}"))?;
    let members = manifest
        .workspace
        .with_context(|| {
            format!("Failed to find [workspace] entry in the cargo manifest at {dir:?}")
        })?
        .members;
    assert!(
        !members.is_empty(),
        "Error: found empty [workspace.members] key - no drivers to build."
    );
    std::env::set_current_dir(cur_dir)?;

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
                arch if arch.starts_with("thumb") => ("lr", "0x8badf00d ^ 1", " ^1", 32),
                arch if arch.starts_with("riscv") => ("ra", "0x8badf00d", "", 32),
                arch if arch.starts_with("x86_64") => ("@[rsp, 8]", "0xffffffff8badf00d", "", 64),
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
{} := {}
replace <__checkct_private_rand>{thumb} () by
res<{size}> := secret
return res
end
replace <__checkct_public_rand>{thumb} () by
res<{size}> := nondet
return res
end
halt at {}
explore all
"#,
                    abi.0,
                    abi.1,
                    abi.1,
                    size = abi.3,
                    thumb = abi.2,
                )
                .as_bytes(),
            )?;

            let binsec_cmd = format!(
    "binsec -sse -checkct -sse-depth 1000000000 -sse-jump-enum 64 -sse-script {} -sse-timeout {} -arm-supported-modes thumb {}",
    dir.join("target").join(target).join(format!("{driver}.binsec")).to_string_lossy(),
    timeout.as_secs(),
    dir.join("target").join(target).join("release").join(&driver).to_string_lossy(),
);

            println!("Running: {binsec_cmd}");
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(&binsec_cmd)
                .output()
                .context("Failed to run binsec")?;

            if !output.status.success() {
                bail!(
                    "Error while running binsec:\nstdout: {}\nstderr: {}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                );
            } else {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("[checkct:result] Program status is : insecure") {
                    println!("INSECURE");
                } else if !stdout.contains("[checkct:result] Program status is : secure") {
                    panic!(
                        "UNEXPECTED:\nstdout: {}\nstderr: {}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }

            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        }
    }

    Ok(())
}
