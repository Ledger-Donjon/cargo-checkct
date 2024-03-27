use std::fs;

use assert_cmd::Command;

// NB: to run, use "cargo test --test subtle_eq -- --exact --nocapture"

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

    let main_path = "tests/subtle_eq/checkct/driver/src/main.rs";
    let mut main_file = fs::read_to_string(main_path).unwrap();
    let idx = main_file.find("// USER CODE GOES HERE").unwrap();
    main_file.insert_str(idx, user_code);
    fs::write(main_path, main_file).unwrap();

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
