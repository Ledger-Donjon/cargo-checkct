use std::{fs, io::Write, path::Path};

use anyhow::{Context, Result};
use cargo_manifest::Manifest;

pub fn init_workspace(path: &Path, name: &str) -> Result<()> {
    // First of all, we need to check that the designated path is a proper cargo lib workspace,
    // and recover the name of the crate
    let manifest = Manifest::from_path(path.join("Cargo.toml"))
        .with_context(|| format!("Failed to find the cargo manifest at: {path:?}"))?;
    let lib_name = manifest
        .lib
        .with_context(|| format!("Failed to find [lib] entry in the cargo manifest at {path:?}"))?
        .name
        .with_context(|| format!("Failed to get crate name in the cargo manifest at {path:?}"))?;
    println!("found library name: {}", lib_name);

    // The workspace directory name is hardcoded to /checkct
    let workspace_dir = path.join("checkct");
    fs::create_dir_all(workspace_dir.join(".cargo"))?;

    // Create the rust-toolchain.toml file
    let mut toolchain_file = fs::File::create(workspace_dir.join("rust-toolchain.toml"))?;
    toolchain_file.write_all(
        r#"[toolchain]
channel = "nightly"
targets = ["thumbv7em-none-eabihf", "riscv32imac-unknown-none-elf", "x86_64-unknown-linux-gnu"]
profile = "minimal"
components = ["rustfmt", "rust-src"]"#
            .as_bytes(),
    )?;

    // Create the workspace Cargo.toml file
    let mut cargo_file = fs::File::create(workspace_dir.join("Cargo.toml"))?;
    cargo_file.write_all(
        format!(
            r#"[workspace]
members = ["{name}"]
resolver = "2"

[profile.release]
debug = true
panic = "abort""#,
        )
        .as_bytes(),
    )?;

    create_driver(&workspace_dir, &lib_name, name)?;

    Ok(())
}

pub fn create_driver(workspace_dir: &Path, lib_name: &str, name: &str) -> Result<()> {
    // Create the crate's directory structure
    fs::create_dir_all(workspace_dir.join(name).join("src"))?;

    // Create the driver Cargo.toml file
    let mut driver_cargo_file = fs::File::create(workspace_dir.join(name).join("Cargo.toml"))?;
    driver_cargo_file.write_all(
        format!(
            r#"[package]
name = "{name}"
version = "0.0.0"
publish = false
edition = "2021"

[dependencies]
rand_core = "0.6.4"

[dependencies.{lib_name}]
path = "../..""#
        )
        .as_bytes(),
    )?;

    // Create the config.toml file
    let mut config_file = fs::File::create(workspace_dir.join(".cargo").join("config.toml"))?;
    config_file.write_all(
        r#"[build]
target = ["thumbv7em-none-eabihf", "riscv32imac-unknown-none-elf", "x86_64-unknown-linux-gnu"]

[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-nostartfiles"]

[target.x86_64-unknown-linux-gnu]
linker = "x86_64-unknown-linux-gnu-gcc"

[unstable]
unstable-options = true
build-std = ["core", "panic_abort"]
build-std-features = ["panic_immediate_abort", "compiler-builtins-mem"]"#
            .as_bytes(),
    )?;

    // Create the driver's rng.rs file
    let mut rng_file = fs::File::create(workspace_dir.join(name).join("src").join("rng.rs"))?;
    rng_file.write_all(
        r#"//----- AUTOGENERATED BY CHECKCT: DO NOT MODIFY -----
//
#[no_mangle]
#[inline(never)]
pub fn __checkct_private_rand() -> u8 {
    unsafe {
        core::ptr::read_volatile(0xcafe as *const u8)
    }
}

#[no_mangle]
#[inline(never)]
pub fn __checkct_public_rand() -> u8 {
    unsafe {
        core::ptr::read_volatile(0xf00d as *const u8)
    }
}

pub use rand_core::{CryptoRng, RngCore};

pub struct PrivateRng;

impl RngCore for PrivateRng {
    fn next_u32(&mut self) -> u32 {
        (__checkct_private_rand() as u32) << 24
            | (__checkct_private_rand() as u32) << 16
            | (__checkct_private_rand() as u32) << 8
            | (__checkct_private_rand() as u32)
    }

    fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for d in dest {
            *d = __checkct_private_rand();
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for PrivateRng {}

pub struct PublicRng;

impl RngCore for PublicRng {
    fn next_u32(&mut self) -> u32 {
        (__checkct_public_rand() as u32) << 24
            | (__checkct_public_rand() as u32) << 16
            | (__checkct_public_rand() as u32) << 8
            | (__checkct_public_rand() as u32)
    }

    fn next_u64(&mut self) -> u64 {
        (self.next_u32() as u64) << 32 | (self.next_u32() as u64)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for d in dest {
            *d = __checkct_public_rand();
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for PublicRng {}"#
            .as_bytes(),
    )?;

    // Create the driver's main.rs file
    let mut main_file = fs::File::create(workspace_dir.join(name).join("src").join("main.rs"))?;
    main_file.write_all(
        r#"#![no_std]
#![no_main]
        
#[no_mangle]
#[inline(never)]
fn __checkct() {
    // USER CODE GOES HERE
}


//----- AUTOGENERATED BY CHECKCT: DO NOT MODIFY -----
//
mod rng;
use rng::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    core::hint::black_box(__checkct());
    panic!()
}
        
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}"#
        .as_bytes(),
    )?;

    Ok(())
}
