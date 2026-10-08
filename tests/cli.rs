//! Tests of the workspace management commands, which do not require binsec.

use std::fs;

mod common;
use common::{cargo_checkct, edit_driver, fixture};

#[test]
fn init_creates_workspace() {
    let (_tmp, lib) = fixture("subtle_eq");
    cargo_checkct()
        .arg("init")
        .arg("--dir")
        .arg(&lib)
        .assert()
        .success();

    let workspace = lib.join("checkct");
    for file in [
        "Cargo.toml",
        "rust-toolchain.toml",
        ".cargo/config.toml",
        "checkct_macros/Cargo.toml",
        "checkct_macros/src/lib.rs",
        "driver/Cargo.toml",
        "driver/src/main.rs",
        "driver/src/rng.rs",
        "driver/src/driver.rs",
    ] {
        assert!(workspace.join(file).is_file(), "missing {file}");
    }

    // The workspace must not depend on the location of the cargo-checkct sources
    let manifest = fs::read_to_string(workspace.join("driver/Cargo.toml")).unwrap();
    assert!(manifest.contains(r#"checkct_macros = { path = "../checkct_macros" }"#));
    assert!(manifest.contains(r#"subtle_eq = { path = "../.." }"#));
}

#[test]
fn init_does_not_overwrite_workspace() {
    let (_tmp, lib) = fixture("subtle_eq");
    cargo_checkct()
        .arg("init")
        .arg("--dir")
        .arg(&lib)
        .assert()
        .success();
    edit_driver(&lib, "driver", |driver| driver + "// precious user code\n");

    cargo_checkct()
        .arg("init")
        .arg("--dir")
        .arg(&lib)
        .assert()
        .failure();
    let driver = fs::read_to_string(lib.join("checkct/driver/src/driver.rs")).unwrap();
    assert!(driver.contains("// precious user code"));
}

#[test]
fn add_preserves_workspace_manifest() {
    let (_tmp, lib) = fixture("subtle_eq");
    // Also check the `cargo checkct ...` invocation form
    cargo_checkct()
        .args(["checkct", "init", "--name", "first"])
        .current_dir(&lib)
        .assert()
        .success();
    let manifest_path = lib.join("checkct/Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path).unwrap();
    fs::write(&manifest_path, manifest + "\n# user customization\n").unwrap();

    cargo_checkct()
        .args(["add", "--name", "second", "--dir"])
        .arg(&lib)
        .assert()
        .success();
    let manifest = fs::read_to_string(&manifest_path).unwrap();
    assert!(
        manifest.contains(r#"members = ["first", "second"]"#),
        "{manifest}"
    );
    assert!(manifest.contains("# user customization"), "{manifest}");
    assert!(lib.join("checkct/second/src/driver.rs").is_file());

    cargo_checkct()
        .args(["add", "--name", "second", "--dir"])
        .arg(&lib)
        .assert()
        .failure();
}

#[test]
fn run_without_workspace() {
    let (_tmp, lib) = fixture("subtle_eq");
    let output = cargo_checkct()
        .arg("run")
        .arg("--dir")
        .arg(&lib)
        .assert()
        .failure();
    let stderr = String::from_utf8_lossy(&output.get_output().stderr);
    assert!(stderr.contains("No checkct workspace found"), "{stderr}");
}
