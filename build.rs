// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    println!("cargo:rustc-cfg=on_apple_silicon");
}
