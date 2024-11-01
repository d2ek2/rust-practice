FROM rust:1.74 as builder

WORKDIR /usr/src/app

# 依存関係のキャッシュ
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm -rf src/

# 実際のソースコードをビルド
COPY src ./src
RUN touch src/main.rs && \
    cargo build --release

# 開発環境ステージ
FROM rust:1.74-slim

WORKDIR /workspace

# シェルをbashに設定
SHELL ["/bin/bash", "-c"]

# デフォルトシェルをbashに設定
ENV SHELL /bin/bash

# 開発に必要なツールのインストール
RUN apt-get update && \
    apt-get install -y git curl && \
    rm -rf /var/lib/apt/lists/*

# cargoのキャッシュディレクトリを作成
RUN mkdir -p /usr/local/cargo/registry

# 非root用ユーザーの作成
ARG USER_NAME=rustdev
ARG USER_ID=1000
RUN useradd -m -u ${USER_ID} ${USER_NAME}
USER ${USER_NAME}
