//! End-to-end tests, which require binsec.

use std::sync::{Mutex, MutexGuard};

mod common;
use common::{cargo_checkct, edit_driver, fixture, init};

/// Each test already builds and verifies drivers in parallel, so run them one at a time
/// to avoid exhausting the memory of the machine.
fn serialize() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

fn run(lib: &std::path::Path, args: &[&str]) -> (bool, String) {
    let output = cargo_checkct()
        .arg("run")
        .arg("--dir")
        .arg(lib)
        .arg("--timeout=60")
        .args(args)
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    println!("{stdout}");
    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    (output.status.success(), stdout)
}

#[test]
fn subtle() {
    let _guard = serialize();
    let (_tmp, lib) = fixture("subtle_eq");
    init(
        &lib,
        r#"use subtle_eq::ConstantTimeEq;
    let mut left = [0u8; 8];
    let mut right = [0x42u8; 8];
    PrivateRng.fill_bytes(&mut left);
    PublicRng.fill_bytes(&mut right);
    core::hint::black_box(left.ct_eq(&right));
    // Large enough to be zeroed by memset, which uses `rep stos` on x86_64
    let mut buffer = [0u8; 4096];
    core::hint::black_box(&mut buffer);"#,
    );

    let (success, stdout) = run(&lib, &[]);
    assert!(success);
    assert!(stdout.contains("SECURE"));
    assert!(!stdout.contains("INSECURE"));
}

#[test]
fn vulnerable() {
    let _guard = serialize();
    let (_tmp, lib) = fixture("vulnerable_eq");
    init(
        &lib,
        r#"use vulnerable_eq::eq;
    let mut left = [0u8; 32];
    let mut right = [0u8; 32];
    PrivateRng.fill_bytes(&mut left);
    PublicRng.fill_bytes(&mut right);
    core::hint::black_box(eq(&left, &right));"#,
    );
    // A second, secure, entrypoint in the same driver, with a panic path that only depends on
    // public values
    edit_driver(&lib, "driver", |driver| {
        driver
            + r#"
#[checkct]
pub fn checkct_xor() {
    assert!(PublicRng.next_u32() != 0);
    core::hint::black_box(PrivateRng.next_u64() ^ PublicRng.next_u64());
}
"#
    });

    let (success, stdout) = run(&lib, &[]);
    assert!(!success);
    assert!(stdout.contains("INSECURE"));
    let summary = stdout.split("Summary:").nth(1).unwrap();
    for target in [
        "thumbv7em-none-eabihf",
        "riscv32imac-unknown-none-elf",
        "x86_64-unknown-linux-gnu",
    ] {
        let entrypoint_status = |entrypoint: &str| {
            summary
                .lines()
                .find(|line| line.contains(target) && line.contains(entrypoint))
                .and_then(|line| line.split_whitespace().last())
                .map(str::to_owned)
        };
        assert_eq!(
            entrypoint_status("driver::driver::checkct ").as_deref(),
            Some("insecure"),
            "{target}"
        );
        assert_eq!(
            entrypoint_status("driver::driver::checkct_xor").as_deref(),
            Some("secure"),
            "{target}"
        );
    }
}

#[test]
fn secret_division() {
    let _guard = serialize();
    let (_tmp, lib) = fixture("secret_division");
    init(
        &lib,
        r#"core::hint::black_box(secret_division::div(PublicRng.next_u32(), PrivateRng.next_u32()));"#,
    );

    // Divisions are not checked by default
    let (success, stdout) = run(&lib, &[]);
    assert!(success);
    assert!(!stdout.contains("INSECURE"));

    let (success, stdout) = run(&lib, &["--checks=control-flow,memory-access,divisor"]);
    assert!(!success);
    assert!(stdout.contains("INSECURE"));
}
