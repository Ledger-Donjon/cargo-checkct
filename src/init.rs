// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::Path;

use anyhow::{Result, bail};

use crate::common::{create_driver, create_macros_crate, get_lib_name, write_file};

pub fn init_workspace(path: &Path, name: &str) -> Result<()> {
    // First of all, we need to check that the designated path is a proper cargo lib workspace,
    // and recover the name of the crate
    let lib_name = get_lib_name(path)?;
    println!("found library name: {lib_name}");

    // The workspace directory name is hardcoded to /checkct
    let workspace_dir = path.join("checkct");
    if workspace_dir.join("Cargo.toml").exists() {
        bail!(
            "A checkct workspace already exists in {workspace_dir:?}; use `cargo-checkct add` to add a new driver to it"
        );
    }

    write_file(
        &workspace_dir.join("rust-toolchain.toml"),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/template/rust-toolchain.toml"
        )),
    )?;

    write_file(
        &workspace_dir.join(".cargo").join("config.toml"),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/template/.cargo/config.toml"
        )),
    )?;

    write_file(
        &workspace_dir.join("Cargo.toml"),
        &format!(
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/template/Cargo.toml")),
            members = format_args!("\"{name}\""),
        ),
    )?;

    create_macros_crate(&workspace_dir)?;
    create_driver(&workspace_dir, &lib_name, name)?;

    Ok(())
}
