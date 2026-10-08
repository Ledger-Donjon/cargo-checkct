# SPDX-FileCopyrightText: 2024 Ledger
#
# SPDX-License-Identifier: MIT OR Apache-2.0

FROM docker.io/library/rust:1-slim-trixie

ARG OCAML_VERSION=5.4.1
ARG BINSEC_VERSION=0.11.3
ARG UNISIM_ARCHISEC_VERSION=0.0.14

ENV OPAMROOT=/opt/opam \
    OPAMYES=1 \
    OPAMCOLOR=never

# Install system dependencies, then binsec, unisim_archisec and bitwuzla from opam
RUN export DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get install -y --no-install-recommends --no-install-suggests \
        bzip2 ca-certificates curl g++ gcc git libgmp-dev libmpfr-dev make opam patch pkgconf unzip && \
    rm -rf /var/lib/apt/lists/* && \
    opam init --bare --disable-sandboxing -n && \
    opam switch create checkct "ocaml-base-compiler.${OCAML_VERSION}" && \
    opam install --switch=checkct \
        "binsec.${BINSEC_VERSION}" "unisim_archisec.${UNISIM_ARCHISEC_VERSION}" bitwuzla-cxx && \
    opam clean -a -c -s --logs

ENV PATH=/opt/opam/checkct/bin:$PATH

# Install the nightly toolchain pinned by the checkct workspace template
COPY template/rust-toolchain.toml /tmp/toolchain/rust-toolchain.toml
RUN cd /tmp/toolchain && rustup toolchain install && rm -rf /tmp/toolchain

# Install cargo-checkct
COPY . /src/
RUN cargo install --locked --path /src --root /usr/local && rm -rf /src/target
WORKDIR /src
