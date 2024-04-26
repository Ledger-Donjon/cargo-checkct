use std::fs;

use assert_cmd::Command;

#[test]
fn subtle() {
    Command::cargo_bin("cargo-checkct")
        .unwrap()
        .arg("init")
        .arg("--dir=tests/subtle_eq")
        .assert()
        .success();

    let user_code = r#"use subtle_eq::ConstantTimeEq;
    let mut left = [0u8; 8];
    let mut right = [0x42u8; 8];
    PrivateRng.fill_bytes(&mut left);
    PublicRng.fill_bytes(&mut right);

    left.ct_eq(&right);"#;

    let driver_path = "tests/subtle_eq/checkct/driver/src/driver.rs";
    let mut driver_file = fs::read_to_string(driver_path).unwrap();
    let idx = driver_file.find("// USER CODE GOES HERE").unwrap();
    driver_file.insert_str(idx, user_code);
    fs::write(driver_path, driver_file).unwrap();

    let output = Command::cargo_bin("cargo-checkct")
        .unwrap()
        .arg("run")
        .arg("--dir=tests/subtle_eq")
        .arg("--timeout=60")
        .assert()
        .success();

    let output = String::from_utf8_lossy(&output.get_output().stdout);
    println!("{output}");
    assert!(!output.contains("INSECURE"));
}
