FROM rust:alpine

RUN apk add --no-cache musl-dev build-base

RUN rustup target add aarch64-apple-darwin && \
    rustup target add aarch64-unknown-linux-gnu && \
    rustup target add x86_64-apple-darwin && \
    rustup target add x86_64-unknown-linux-gnu && \
    rustup target add x86_64-unknown-linux-musl

CMD cargo build --release --target aarch64-apple-darwin && \
    cargo build --release --target aarch64-unknown-linux-gnu && \
    cargo build --release --target x86_64-apple-darwin && \
    cargo build --release --target x86_64-unknown-linux-gnu && \
    cargo build --release --target x86_64-unknown-linux-musl
