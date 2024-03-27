# cargo-checkct

## Install

You will need to install libgmp and gcc/g++ first, as well as opam (see <https://opam.ocaml.org/doc/Install.html>).
You will also need rust of course (<https://rustup.rs/>).

```console
opam init -y
opam install -y dune dune-site menhir grain_dypgen ocamlgraph zarith toml bitwuzla
eval $(opam env)
git clone https://github.com/binsec/binsec
git clone https://github.com/binsec/unisim_archisec
pushd unisim_archisec && git apply ../unisim.patch && dune build @install && dune install && popd
pushd binsec && git apply ../binsec.patch && dune build @install && dune install && popd
```

## Usage

Running

```console
cargo run --release -- init -d <path/to/your/rust/crypto/library>
```

will initialize a `checkct/` directory at the designated path, and inside it a `driver` crate.
You can then implement your verification harness (which checkct calls a driver) in `checkct/driver/src/main.rs`.
You can change the rustc targets for which verification will be done by modifying `checkct/.cargo/config.toml` and `checkct/rust-toolchain.toml`.

At anypoint, you can add an additional verification driver (to verify another function exposed by your library, typically) with

```console
cargo run --release -- add -d <path/to/your/rust/crypto/library> -n <name_of_the_new_driver_crate>
```

Then running

```console
cargo run --release -- run -d <path/to/your/rust/crypto/library>
```

will build all drivers (for all the respective targets) in `release` mode and run binsec on the resulting binaries.

## Examples

You can find simple examples of the above in `tests`, as well as life-sized examples in `examples`. Of particular interest might be the `masked_aes` example, which shows how to analyse a C or asm library, by simply exposing a thin rust API for the functions to test.

## License

Licensed under the Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>) or the MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>), at your option.
