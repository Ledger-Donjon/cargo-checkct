#![allow(dead_code)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use assert_cmd::Command;
use tempfile::TempDir;

/// Copy the fixture library `name` to a new temporary directory, and return its path.
pub fn fixture(name: &str) -> (TempDir, PathBuf) {
    let tmp = tempfile::tempdir().unwrap();
    let lib = tmp.path().join(name);
    copy_dir(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name),
        &lib,
    );
    (tmp, lib)
}

fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &dst.join(entry.file_name()));
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name())).unwrap();
        }
    }
}

pub fn cargo_checkct() -> Command {
    let mut cmd = Command::cargo_bin("cargo-checkct").unwrap();
    // Each checkct workspace must be built in its own target directory
    cmd.env_remove("CARGO_TARGET_DIR");
    cmd
}

/// Create a checkct workspace for `lib`, with `code` as the body of the driver entrypoint.
pub fn init(lib: &Path, code: &str) {
    cargo_checkct()
        .arg("init")
        .arg("--dir")
        .arg(lib)
        .assert()
        .success();
    edit_driver(lib, "driver", |driver| {
        driver.replace("// USER CODE GOES HERE", code)
    });
}

pub fn edit_driver(lib: &Path, driver: &str, edit: impl FnOnce(String) -> String) {
    let path = lib
        .join("checkct")
        .join(driver)
        .join("src")
        .join("driver.rs");
    let contents = fs::read_to_string(&path).unwrap();
    fs::write(path, edit(contents)).unwrap();
}
