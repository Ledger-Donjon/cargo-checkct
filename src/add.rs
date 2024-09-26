// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{fs, io::Write, path::Path};

use anyhow::{bail, Result};

use crate::common::{create_driver, get_lib_name, get_workspace_members};

pub fn add_driver(path: &Path, name: &str) -> Result<()> {
    let name = name.to_owned();
    let workspace_dir = path.join("checkct");

    // First recover the library name and the members of the checkct workspace
    let lib_name = get_lib_name(path)?;
    println!("found library name: {}", lib_name);

    let mut members = get_workspace_members(&workspace_dir)?;
    if members.contains(&name) {
        bail!("Error: the checkct workspace already contains driver {name}")
    }

    // Then create the actual driver
    create_driver(&workspace_dir, &lib_name, &name)?;

    // Finally, add the newly created driver to the checkct workspace
    members.push(name);
    let mut cargo_file = fs::File::create(workspace_dir.join("Cargo.toml"))?;
    cargo_file.write_all(
        format!(
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/template/Cargo.toml")),
            members = members
                .into_iter()
                .map(|s| format!("\"{s}\""))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .as_bytes(),
    )?;

    Ok(())
}
