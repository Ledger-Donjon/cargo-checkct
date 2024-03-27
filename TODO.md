# TODO

- [X] Have `init` generate a workspace with a single `driver` binary at first
- [X] Have `run` look at the workspace and its members' manifests to recover the executables' names after compilation
- [X] Add Risc-V support
- [X] Patch binsec and enable RISCV compressed instructions
- [X] Systematically add rand_core and PublicRng, Private Rng to generated drivers
- [X] Add an `add` command to add a new binary to the `checkct` workspace
- [ ] Make sure it works both on aarch64 MacOS, and on amd64 Linux and Windows (paths should be OK, but linker directives will differ)
