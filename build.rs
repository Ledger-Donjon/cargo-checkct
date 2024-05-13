// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    println!("cargo::rustc-check-cfg=cfg(on_apple_silicon)");

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    println!("cargo:rustc-cfg=on_apple_silicon");
}
