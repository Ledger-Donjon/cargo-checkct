use std::{fs, io::Write, path::Path};

use anyhow::{Context, Result};
use cargo_manifest::Manifest;

pub fn add_driver(path: &Path, name: &str) -> Result<()> {
    let workspace_dir = path.join("checkct");

    // Read the workspace Cargo.toml file
    let manifest = Manifest::from_path(workspace_dir.join("Cargo.toml"))
        .with_context(|| format!("Failed to find the cargo manifest at: {workspace_dir:?}"))?;
    let mut members = manifest
        .workspace
        .with_context(|| {
            format!("Failed to find [workspace] entry in the cargo manifest at {workspace_dir:?}")
        })?
        .members;
    members.push(name.to_owned());

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
