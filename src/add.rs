// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use toml_edit::{Array, DocumentMut};

use crate::common::{create_driver, get_lib_name, get_workspace_members};

pub fn add_driver(path: &Path, name: &str) -> Result<()> {
    let workspace_dir = path.join("checkct");

    // First recover the library name and the members of the checkct workspace
    let lib_name = get_lib_name(path)?;
    println!("found library name: {lib_name}");

    let members = get_workspace_members(&workspace_dir)?;
    if members.iter().any(|member| member == name) || workspace_dir.join(name).exists() {
        bail!("Error: the checkct workspace already contains driver {name}")
    }

    // Then create the actual driver
    create_driver(&workspace_dir, &lib_name, name)?;

    // Finally, add the newly created driver to the checkct workspace, preserving
    // any other modification made to the workspace manifest
    let manifest_path = workspace_dir.join("Cargo.toml");
    let mut manifest = fs::read_to_string(&manifest_path)
        .with_context(|| format!("Failed to read {manifest_path:?}"))?
        .parse::<DocumentMut>()
        .with_context(|| format!("Failed to parse {manifest_path:?}"))?;
    let workspace_members = manifest["workspace"]
        .as_table_like_mut()
        .context("[workspace] is not a table")?
        .entry("members")
        .or_insert(toml_edit::value(Array::new()))
        .as_array_mut()
        .context("[workspace.members] is not an array")?;
    workspace_members.push(name);
    fs::write(&manifest_path, manifest.to_string())
        .with_context(|| format!("Failed to write {manifest_path:?}"))?;

    Ok(())
}
