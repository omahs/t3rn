# syntax=docker/dockerfile:1

FROM docker.io/library/ubuntu:22.04

LABEL description="paritytech/srtool image including a t3rn registry login"

ARG GITHUB_PERSONAL_ACCESS_TOKEN
ARG T3RN_CARGO_REGISTRY_TOKEN
ARG REGISTRY_INDEX_REPO
ENV RUSTC_VERSION="1.62.0"
ENV DOCKER_IMAGE="t3rn/srtool"
ENV PROFILE=release
ENV PACKAGE=polkadot-runtime
ENV BUILDER=builder
ARG UID=1001
ARG GID=1001
ENV SRTOOL_TEMPLATES=/srtool/templates

RUN groupadd -g $GID $BUILDER && \
    useradd --no-log-init  -m -u $UID -s /bin/bash -d /home/$BUILDER -r -g $BUILDER $BUILDER
RUN mkdir -p $SRTOOL_TEMPLATES && \
    mkdir /build && chown -R $BUILDER /build && \
    mkdir /out && chown -R $BUILDER /out

WORKDIR /tmp
ENV DEBIAN_FRONTEND=noninteractive

# Tooling
ARG SUBWASM_VERSION=0.18.0
ARG TERA_CLI_VERSION=0.2.1
ARG TOML_CLI_VERSION=0.2.1

RUN apt update && \
    apt upgrade -y && \
    apt install --no-install-recommends -y \
        cmake pkg-config libssl-dev make \
        git clang bsdmainutils ca-certificates curl && \
    curl -L https://github.com/stedolan/jq/releases/download/jq-1.6/jq-linux64 --output /usr/bin/jq && chmod a+x /usr/bin/jq && \
    rm -rf /var/lib/apt/lists/* /tmp/* && apt clean

RUN curl -L https://github.com/chevdor/subwasm/releases/download/v$SUBWASM_VERSION/subwasm_linux_amd64_v$SUBWASM_VERSION.deb --output subwasm.deb && dpkg -i subwasm.deb && subwasm --version && \
    curl -L https://github.com/chevdor/tera-cli/releases/download/v$TERA_CLI_VERSION/tera-cli_linux_amd64.deb --output tera_cli.deb && dpkg -i tera_cli.deb && tera --version && \
    curl -L https://github.com/chevdor/toml-cli/releases/download/v$TOML_CLI_VERSION/toml_linux_amd64_v$TOML_CLI_VERSION.deb --output toml.deb && dpkg -i toml.deb && toml --version && \
    rm -rf /tmp/*

RUN git clone --depth 1 https://github.com/paritytech/srtool /tmp/srtool && \
    cp /tmp/srtool/scripts/* /srtool/ && \
    cp /tmp/srtool/templates/* /srtool/templates/ && \
    echo 1.62.0 > /srtool/RUSTC_VERSION && \
    echo 0.9.21 > /srtool/VERSION && \
    rm -rf /tmp/srtool

USER $BUILDER
ENV RUSTUP_HOME="/home/${BUILDER}/rustup"
ENV CARGO_HOME="/home/${BUILDER}/cargo"
ENV PATH="/srtool:$PATH"

RUN echo $SHELL && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . $CARGO_HOME/env && \
    rustup toolchain add stable $RUSTC_VERSION && \
    rustup target add wasm32-unknown-unknown --toolchain $RUSTC_VERSION && \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME && \
    rustup show && rustc -V

RUN git config --global --add safe.directory /build && \
    /srtool/version && \
    echo 'PATH=".:$HOME/cargo/bin:$PATH"' >> $HOME/.bashrc && \
    echo -e "[registries]\nt3rn={index=\"https://github.com/t3rn/registry-index.git\",token=\"$GITHUB_PERSONAL_ACCESS_TOKEN\"}" > $CARGO_HOME/config.toml && \
    $CARGO_HOME/bin/cargo login --registry=t3rn $T3RN_CARGO_REGISTRY_TOKEN

VOLUME [ "/build", "$CARGO_HOME", "/out" ]
WORKDIR /srtool

CMD ["/srtool/build"]