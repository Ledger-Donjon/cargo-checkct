FROM docker.io/library/rust:slim-bullseye

# Install dependencies and Rust toolchains
# Create a symlink for "x86_64-unknown-linux-gnu-gcc" to fix using x86_64-unknown-linux-gnu toolchain.
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get install -y --no-install-recommends --no-install-suggests git libgmp-dev gcc g++ opam pkgconf && \
    apt-get clean && \
    rustup target add thumbv7em-none-eabihf && \
    rustup target add riscv32imac-unknown-none-elf && \
    opam init --disable-sandboxing -y && \
    opam install -y dune dune-site menhir grain_dypgen ocamlgraph zarith toml bitwuzla && \
    echo 'eval $(opam env)' >> /root/.bashrc && \
    ln -s /usr/bin/x86_64-linux-gnu-gcc /usr/local/bin/x86_64-unknown-linux-gnu-gcc

# Install binsec
COPY unisim.patch /opt/

RUN git clone --depth 1 --branch 0.0.8 https://github.com/binsec/unisim_archisec /opt/unisim_archisec && \
    (cd /opt/unisim_archisec && git apply /opt/unisim.patch && eval $(opam env) && dune build @install && dune install) && \
    git clone --depth 1 --branch 0.9.1 https://github.com/binsec/binsec /opt/binsec && \
    (cd /opt/binsec && eval $(opam env) && dune build @install && dune install)

# Copy cargo-checkct to /src
COPY . /src/
WORKDIR /src
