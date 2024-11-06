FROM rust:1.74-slim

WORKDIR /workspace

# 必要なパッケージのインストール
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Rustツールチェーンのインストール
RUN rustup component add \
    rust-analyzer \
    rust-src \
    rustfmt \
    clippy

# シェルとキャッシュの設定
SHELL ["/bin/bash", "-c"]
ENV SHELL /bin/bash

# cargoのキャッシュディレクトリを作成
RUN mkdir -p /usr/local/cargo/registry

# 非root用ユーザーの作成
ARG USER_NAME=rustdev
ARG USER_ID=1000
ARG GROUP_NAME=rustdev
ARG GROUP_ID=1000

RUN groupadd -g ${GROUP_ID} ${GROUP_NAME} && \
    useradd -m -u ${USER_ID} -g ${GROUP_ID} ${USER_NAME} && \
    chown -R ${USER_NAME}:${GROUP_NAME} /workspace /usr/local/cargo

# Cargoの設定
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:$PATH

# ユーザー切り替え
USER ${USER_NAME}

# 作業ディレクトリの権限設定
RUN mkdir -p /workspace/target && \
    chown -R ${USER_NAME}:${GROUP_NAME} /workspace

# デフォルトコマンド
CMD ["bash"]