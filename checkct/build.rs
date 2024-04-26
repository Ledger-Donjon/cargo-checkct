use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

struct Abi<'a> {
    thumb: &'a str,
    size: usize,
}

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_sub = env::var("CARGO_CFG_TARGET_FEATURE").unwrap();

    let abi = match target_arch.as_str() {
        "arm" => if target_sub.contains("thumb") {
                Abi {
                    thumb: " ^1",
                    size: 32,
                }
            } else {
                panic!("unexpected target");
            }
        "riscv32" => Abi {
            thumb: "",
            size: 32,
        },
        "x86_64" => Abi {
            thumb: "",
            size: 64,
        },
        _ => panic!("unexpected target: {target_arch}"),
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bwerk
    // place 'driver.binsec' in 'target/<target_triple>/'
    let mut out_file = fs::File::create(out_dir.join("../../../../driver.binsec")).unwrap();

    out_file.write_all(
        format!(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/driver.binsec.template"
            )),
            size = abi.size,
            thumb = abi.thumb,
        )
        .as_bytes(),
    ).unwrap();
}
