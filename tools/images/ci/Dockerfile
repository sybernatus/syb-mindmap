FROM ubuntu:24.04 as builder

ARG extra_tools="inkscape buildah"

ARG cargo_edit_version="0.13.2"
ARG cargo_binstall_version="1.12.3"
ARG cargo_dioxus_cli_version="0.6.3"
ARG pandoc_version="3.6.4"
ARG proto_version="0.49.1"

# Copy setup scripts
ENV HOME=/root

# Dependencies install
RUN apt-get update \
    && apt-get install -y bash curl git unzip gzip xz-utils build-essential pkg-config libssl-dev \
    && apt-get install -y ${extra_tools} \
    && rm -rf /var/lib/apt/lists/* \

# Rust install
ENV PATH="${HOME}/.cargo/bin:$PATH"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --no-modify-path \
    && ${HOME}/.cargo/bin/cargo install cargo-edit --version "${cargo_edit_version}" \
    && ${HOME}/.cargo/bin/cargo install cargo-binstall --version "${cargo_binstall_version}" \
    && ${HOME}/.cargo/bin/cargo install dioxus-cli --version "${cargo_dioxus_cli_version}"

# Pandoc install \
RUN curl --location "https://github.com/jgm/pandoc/releases/download/${pandoc_version}/pandoc-${pandoc_version}-linux-amd64.tar.gz" | tar -xz -C "." \
  && mv "pandoc-${pandoc_version}/bin/pandoc" "/usr/bin/pandoc" \
  && rm --recursive --force ".bin/pandoc-${pandoc_version}/bin"

# Proto install
ENV PATH="${HOME}/.proto/bin:${HOME}/.proto/shims/:$PATH"
RUN bash -c "bash <(curl -fsSL https://moonrepo.dev/install/proto.sh) '${proto_version}' --yes"
COPY .prototools .prototools
RUN proto install