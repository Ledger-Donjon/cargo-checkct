// SPDX-FileCopyrightText: 2024 Ledger
//
// SPDX-License-Identifier: MIT OR Apache-2.0

fn main() {
    let dir = std::path::Path::new("better_cortexm");

    cc::Build::new()
        .file(dir.join("src").join("3").join("bitslicing.s"))
        .file(dir.join("src").join("3").join("masked_and.s"))
        .file(dir.join("src").join("3").join("masked_xor.s"))
        .file(dir.join("src").join("3").join("masked_mixcolumns.s"))
        .file(dir.join("src").join("3").join("masked_shiftrows.s"))
        .file(dir.join("src").join("3").join("masked_rotword_xorcol.s"))
        .file(dir.join("src").join("3").join("masked_sbox_lin.s"))
        .file(dir.join("src").join("xoshiro.c"))
        .file(dir.join("src").join("masking.c"))
        .file(dir.join("src").join("masked_utils.c"))
        .file(dir.join("src").join("masked_aes.c"))
        .file(dir.join("src").join("masked_aes_sbox.c"))
        .file(dir.join("src").join("masked_aes_keyschedule.c"))
        .include(dir.join("headers"))
        .define("D", "3")
        .define("R", "5")
        .flag("-mcpu=cortex-m4")
        .compile("aes");
}
