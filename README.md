# cargo-checkct

cargo-checkct aims to make it easy for cryptography libraries developers and maintainers to formally verify that their code gets compiled down to constant-time machine code, in CI.

It does so by leveraging [binsec](https://github.com/binsec/binsec), which is developed by the [Binsec team at the CEA](https://binsec.github.io/).

There are, however, a number of limitations to have in mind if you plan on using cargo-checkct, most notably:

- As currently designed, cargo-checkct focuses exclusively on `#![no_std]` libraries or features.
- For the moment, only bare-metal `thumb` and `riscv32` are supported, as well as `x86_64-unknown-linux-gnu` (with some caveats, for instance the use of `cpuid` for runtime detection of cpu features, as used in [RustCrypto](https://github.com/RustCrypto/utils/tree/master/cpufeatures), is not supported). This is mainly due to gaps in the architecture coverage of binsec and [unisim_archisec](https://github.com/binsec/unisim_archisec), that may resorb in the future.
- The analysis is predicated on the fact that all instructions have data-independent timing (meaning for instance that even multiplication and division instructions' timing is not dependent on the operands).
- Only Apple-silicon MacOS and am64 Linux hosts are supported (the only difference being the linker directive passed down to `rustc` for the `x86_64-unknown-linux-gnu` target).

## Install

You will need to install libgmp and gcc/g++ first, as well as opam (see <https://opam.ocaml.org/doc/Install.html>).
You will also need rust of course (<https://rustup.rs/>).

```console
opam init -y
opam install -y dune dune-site menhir grain_dypgen ocamlgraph zarith toml bitwuzla
eval $(opam env)
git clone --branch 0.9.0 https://github.com/binsec/binsec
git clone --branch 0.0.8 https://github.com/binsec/unisim_archisec
pushd unisim_archisec && git apply ../unisim.patch && dune build @install && dune install && popd
pushd binsec && dune build @install && dune install && popd
```

## Usage

Running

```console
cargo b --release
./target/release/cargo-checkct init -d <path/to/your/rust/crypto/library>
```

will initialize a `checkct/` workspace at the designated path, and inside it a `driver` crate.
You can then implement your verification harness (which checkct calls a driver) in `checkct/driver/src/driver.rs`.
You can change the rustc targets for which verification will be done by modifying `checkct/.cargo/config.toml` and `checkct/rust-toolchain.toml`.

At anypoint, you can add an additional verification driver (to verify another function exposed by your library, for instance) with

```console
./target/release/cargo-checkct add -d <path/to/your/rust/crypto/library> -n <name_of_the_new_driver_crate>
```

Then running

```console
./target/release/cargo-checkct run -d <path/to/your/rust/crypto/library>
```

will build all drivers (for all the respective targets) in `release` mode and run binsec on the resulting binaries.

## Examples

You can find simple examples of the above in `tests`, as well as life-sized examples in `examples`. Of particular interest might be the `masked_aes` example, which shows how to analyse a C or asm library, by simply exposing a thin rust API for the functions to test.

## License

Licensed under the Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>) or the MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>), at your option.
