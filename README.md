# cargo-checkct

cargo-checkct aims to make it easy for cryptography libraries developers and maintainers to formally verify that their code gets compiled down to constant-time machine code, in CI.

It does so by leveraging [binsec](https://github.com/binsec/binsec), which is developed by the [Binsec team at the CEA](https://binsec.github.io/).

There are, however, a number of limitations to have in mind if you plan on using cargo-checkct, most notably:

- As currently designed, cargo-checkct focuses exclusively on `#![no_std]` libraries or features.
- For the moment, only bare-metal `thumb` and `riscv32` are supported, as well as `x86_64-unknown-linux-gnu` (with some caveats, for instance `cpuid` is modelled as reporting no optional cpu feature, so only the baseline code paths of runtime cpu feature detection, as used in [RustCrypto](https://github.com/RustCrypto/utils/tree/master/cpufeatures), are verified). This is mainly due to gaps in the architecture coverage of binsec and [unisim_archisec](https://github.com/binsec/unisim_archisec), that may resorb in the future.
- By default, the analysis is predicated on the fact that all instructions have data-independent timing (meaning for instance that even multiplication and division instructions' timing is not dependent on the operands). binsec can also check multiplications and divisions, see `--checks` below.
- Linux and macOS hosts are supported.

## Install

cargo-checkct requires binsec >= 0.11, which can be installed with [opam](https://opam.ocaml.org/doc/Install.html) (you will also need libgmp and gcc/g++):

```console
opam init -y
opam install -y binsec unisim_archisec bitwuzla-cxx
eval $(opam env)
```

You will also need rust of course (<https://rustup.rs/>). Then install cargo-checkct with:

```console
cargo install --locked --git https://github.com/Ledger-Donjon/cargo-checkct cargo-checkct
```

Alternatively, the `Containerfile` builds an image with binsec and cargo-checkct installed.

## Usage

Running

```console
cargo checkct init -d <path/to/your/rust/crypto/library>
```

will initialize a `checkct/` workspace at the designated path, and inside it a `driver` crate.
You can then implement your verification harness (which checkct calls a driver) in `checkct/driver/src/driver.rs`:
each function annotated with `#[checkct]` is verified, with `PrivateRng` providing secret values and `PublicRng` providing public values.
They provide `next_u32`, `next_u64` and `fill_bytes` methods, and implement the traits of `rand_core` 0.6, 0.9 and 0.10,
so that they can be passed to the functions of your library that need an RNG.
Panics end the analysis of a path, but a panic that depends on a secret (e.g. a bounds check on a secret index) is still reported as a leak.

You can change the rustc targets for which verification will be done by modifying `checkct/.cargo/config.toml`.
The drivers are built with the nightly toolchain pinned in `checkct/rust-toolchain.toml`.

At anypoint, you can add an additional verification driver (to verify another function exposed by your library, for instance) with

```console
cargo checkct add -d <path/to/your/rust/crypto/library> -n <name_of_the_new_driver_crate>
```

Then running

```console
cargo checkct run -d <path/to/your/rust/crypto/library>
```

will build all drivers (for all the respective targets) in `release` mode and run binsec on the resulting binaries.
The main options of `run` are:

- `--timeout <SECONDS>`: binsec timeout for each entrypoint (600 seconds by default).
- `--skip-unknown`: do not fail if binsec cannot conclude.
- `--all-leaks`: by default, the verification of an entrypoint stops at its first leak; with this option, binsec keeps exploring to report all the leaky instructions (which can take much longer).
- `--checks <CHECKS>`: the comma-separated list of constant-time checks to perform, among `control-flow`, `memory-access`, `multiplication`, `dividend` and `divisor` (`control-flow,memory-access` by default).
  The multiplication and division checks are experimental, but variable-time division is a [real threat](https://kyberslash.cr.yp.to/), so consider enabling `dividend,divisor`.
- `--jobs <N>`: the number of binsec analyses to run in parallel (the number of CPUs by default).

## GitHub Actions

This repository is also a GitHub Action, which installs binsec (caching it across runs) and cargo-checkct, and runs `cargo checkct run`:

```yml
permissions:
  contents: read

jobs:
  checkct:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5
      - uses: Ledger-Donjon/cargo-checkct@main # or a specific commit
        with:
          directory: .  # path to the library containing the checkct workspace
          # timeout: 600
          # checks: control-flow,memory-access
          # skip-unknown: false
          # all-leaks: false
```

A summary of the results is added to the job summary.

## Migrating from older versions

Workspaces created by older versions of cargo-checkct do not work with recent nightly toolchains and binsec versions.
The simplest way to migrate is to move your `driver.rs` files (and any other file you modified) out of the way,
delete the `checkct` directory, run `cargo checkct init` (and `cargo checkct add` for additional drivers), and move your files back.
`RngCore` and `CryptoRng` no longer need to be imported in `driver.rs`.

## Examples

You can find simple examples of the above in `tests`, as well as life-sized examples in `examples`. Of particular interest might be the `masked_aes` example, which shows how to analyse a C or asm library, by simply exposing a thin rust API for the functions to test.

## License

Licensed under the Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>) or the MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>), at your option.
