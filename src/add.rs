use std::{fs, io::Write, path::Path};

use anyhow::{Context, Result};
use cargo_manifest::Manifest;

pub fn add_driver(path: &Path, name: &str) -> Result<()> {
    let name = name.to_owned();
    let workspace_dir = path.join("checkct");

    // First recover the library name and the members of the checkct workspace
    let manifest = Manifest::from_path(path.join("Cargo.toml"))
        .with_context(|| format!("Failed to find the cargo manifest at: {path:?}"))?;
    let lib_name = manifest
        .lib
        .with_context(|| format!("Failed to find [lib] entry in the cargo manifest at {path:?}"))?
        .name
        .with_context(|| format!("Failed to get crate name in the cargo manifest at {path:?}"))?;
    println!("found library name: {}", lib_name);

    let manifest = Manifest::from_path(workspace_dir.join("Cargo.toml"))
        .with_context(|| format!("Failed to find the cargo manifest at: {workspace_dir:?}"))?;
    let mut members = manifest
        .workspace
        .with_context(|| {
            format!("Failed to find [workspace] entry in the cargo manifest at {workspace_dir:?}")
        })?
        .members;
    assert!(
        !members.contains(&name),
        "Error: the checkct workspace already contains driver {name}"
    );

    // Then create the actual driver
    crate::init::create_driver(&workspace_dir, &lib_name, &name)?;

    // Finally, add the newly created driver to the checkct workspace
    members.push(name);
    let mut cargo_file = fs::File::create(workspace_dir.join("Cargo.toml"))?;
    cargo_file.write_all(
        format!(
            r#"[workspace]
members = [{}]
resolver = "2"

[profile.release]
debug = true
panic = "abort""#,
            members
                .into_iter()
                .map(|s| format!("\"{s}\""))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .as_bytes(),
    )?;

    Ok(())
}
