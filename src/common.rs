// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{fs, io::Write, path::Path};

use anyhow::{Context, Result, anyhow};
use cargo_manifest::Manifest;

/// Retrieve the name of the rust library at `path` from its cargo manifest.
pub fn get_lib_name(path: &Path) -> Result<String> {
    let manifest = Manifest::from_path(path.join("Cargo.toml"))
        .with_context(|| format!("Failed to find the cargo manifest at: {path:?}"))?;

    // We recover the package name directly, not the lib.name entry, because
    // the latter has dashes '-' replaced with underscores '_', but we need the actual, unaltered name
    let lib_name = manifest
        .package
        .with_context(|| format!("Failed to find package entry in the cargo manifest at {path:?}"))?
        .name;

    Ok(lib_name)
}

/// Retrieve the members of the cargo workspace at `workspace_dir`.
pub fn get_workspace_members(workspace_dir: &Path) -> Result<Vec<String>> {
    let manifest = Manifest::from_path(workspace_dir.join("Cargo.toml"))
        .with_context(|| format!("Failed to find the cargo manifest at: {workspace_dir:?}"))?;
    let members = manifest
        .workspace
        .with_context(|| {
            format!("Failed to find [workspace] entry in the cargo manifest at {workspace_dir:?}")
        })?
        .members;

    Ok(members)
}

/// Create a driver crate named `name` in the checkct workspace at `workspace_dir`, to test the `lib_name` crate.
pub fn create_driver(workspace_dir: &Path, lib_name: &str, name: &str) -> Result<()> {
    // Create the crate's directory structure
    let driver_path = workspace_dir.join(name);
    fs::create_dir_all(driver_path.join("src"))?;

    // Relative path to checkct_macros
    let checkct_macros_crate_path = pathdiff::diff_paths(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("checkct_macros"),
        driver_path.canonicalize()?,
    )
    .context("Failed to compute the relative path between checkct_macros and the driver directory")?
    .into_os_string()
    .into_string()
    .map_err(|_| anyhow!("Failed to convert relative checkct_macros path to string"))?;

    // Create the driver Cargo.toml file
    let mut driver_cargo_file = fs::File::create(driver_path.join("Cargo.toml"))?;
    driver_cargo_file.write_all(
        format!(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/template/driver/Cargo.toml"
            )),
            name = name,
            checkct_macros_crate_path = checkct_macros_crate_path,
            lib_name = lib_name
        )
        .as_bytes(),
    )?;

    // Create the driver's rng.rs file
    let mut rng_file = fs::File::create(workspace_dir.join(name).join("src").join("rng.rs"))?;
    rng_file.write_all(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/template/driver/src/rng.rs"
        ))
        .as_bytes(),
    )?;

    // Create the driver's main.rs file
    let mut main_file = fs::File::create(workspace_dir.join(name).join("src").join("main.rs"))?;
    main_file.write_all(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/template/driver/src/main.rs"
        ))
        .as_bytes(),
    )?;

    // Create the driver's driver.rs file
    let mut driver_file = fs::File::create(workspace_dir.join(name).join("src").join("driver.rs"))?;
    driver_file.write_all(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/template/driver/src/driver.rs"
        ))
        .as_bytes(),
    )?;

    Ok(())
}
