FROM rust:latest

RUN rustup target add aarch64-apple-darwin

CMD cargo build --release \
    --target aarch64-apple-darwin \
    --target aarch64-unknown-linux-gnu \
    --target x86_64-apple-darwin \
    --target x86_64-unknown-linux-gnu
