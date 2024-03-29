use std::{fs, io::Write, path::Path};

use anyhow::Result;

use crate::common::{create_driver, get_lib_name};

pub fn init_workspace(path: &Path, name: &str) -> Result<()> {
    // First of all, we need to check that the designated path is a proper cargo lib workspace,
    // and recover the name of the crate
    let lib_name = get_lib_name(path)?;
    println!("found library name: {}", lib_name);

    // The workspace directory name is hardcoded to /checkct
    let workspace_dir = path.join("checkct");
    fs::create_dir_all(workspace_dir.join(".cargo"))?;

    // Create the rust-toolchain.toml file
    let mut toolchain_file = fs::File::create(workspace_dir.join("rust-toolchain.toml"))?;
    toolchain_file.write_all(
        r#"[toolchain]
channel = "nightly"
targets = ["thumbv7em-none-eabihf", "riscv32imac-unknown-none-elf", "x86_64-unknown-linux-gnu"]
profile = "minimal"
components = ["rustfmt", "rust-src"]"#
            .as_bytes(),
    )?;

    // Create the config.toml file
    let mut config_file = fs::File::create(workspace_dir.join(".cargo").join("config.toml"))?;
    config_file.write_all(
        format!(
            r#"[build]
target = ["thumbv7em-none-eabihf", "riscv32imac-unknown-none-elf", "x86_64-unknown-linux-gnu"]

[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-nostartfiles"]

[target.x86_64-unknown-linux-gnu]
{linker}

[unstable]
unstable-options = true
build-std = ["core", "panic_abort"]
build-std-features = ["panic_immediate_abort", "compiler-builtins-mem"]"#,
            linker = if cfg!(on_apple_silicon) {
                "linker = \"x86_64-unknown-linux-gnu-gcc\""
            } else {
                ""
            }
        )
        .as_bytes(),
    )?;

    // Create the workspace Cargo.toml file
    let mut cargo_file = fs::File::create(workspace_dir.join("Cargo.toml"))?;
    cargo_file.write_all(
        format!(
            r#"[workspace]
members = ["{name}"]
resolver = "2"

[profile.release]
debug = true
panic = "abort""#,
        )
        .as_bytes(),
    )?;

    create_driver(&workspace_dir, &lib_name, name)?;

    Ok(())
}
