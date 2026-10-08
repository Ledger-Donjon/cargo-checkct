// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{fs, path::Path};

use anyhow::{Context, Result};
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

/// Write `contents` to `path`, creating the parent directories if needed.
pub fn write_file(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {parent:?}"))?;
    }
    fs::write(path, contents).with_context(|| format!("Failed to write {path:?}"))
}

/// Vendor the `checkct_macros` crate into the checkct workspace at `workspace_dir`,
/// so that the workspace does not depend on the location of the cargo-checkct sources.
pub fn create_macros_crate(workspace_dir: &Path) -> Result<()> {
    let macros_dir = workspace_dir.join("checkct_macros");
    write_file(
        &macros_dir.join("Cargo.toml"),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/template/checkct_macros/Cargo.toml"
        )),
    )?;
    write_file(
        &macros_dir.join("src").join("lib.rs"),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/checkct_macros/src/lib.rs"
        )),
    )
}

/// Create a driver crate named `name` in the checkct workspace at `workspace_dir`, to test the `lib_name` crate.
pub fn create_driver(workspace_dir: &Path, lib_name: &str, name: &str) -> Result<()> {
    let driver_path = workspace_dir.join(name);

    // Workspaces created by older versions of cargo-checkct referenced the
    // checkct_macros crate in the cargo-checkct sources, so vendor it if needed.
    if !workspace_dir.join("checkct_macros").exists() {
        create_macros_crate(workspace_dir)?;
    }

    write_file(
        &driver_path.join("Cargo.toml"),
        &format!(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/template/driver/Cargo.toml"
            )),
            name = name,
            lib_name = lib_name
        ),
    )?;

    for (file, contents) in [
        (
            "rng.rs",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/template/driver/src/rng.rs"
            )),
        ),
        (
            "main.rs",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/template/driver/src/main.rs"
            )),
        ),
        (
            "driver.rs",
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/template/driver/src/driver.rs"
            )),
        ),
    ] {
        write_file(&driver_path.join("src").join(file), contents)?;
    }

    Ok(())
}
