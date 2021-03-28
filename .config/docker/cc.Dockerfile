# This doesn't currently work. I'm not sure why.

# I think it's probably my host machine being an M1 resulting
# in cross compilation being a problem.

FROM rust:alpine

RUN apk add --no-cache musl-dev build-base && \
    rustup default nightly && \
    rustup update

RUN rustup target add aarch64-apple-darwin && \
    rustup target add aarch64-unknown-linux-gnu && \
    rustup target add x86_64-apple-darwin && \
    rustup target add x86_64-unknown-linux-gnu && \
    rustup target add x86_64-unknown-linux-musl

CMD cargo build --release --target aarch64-unknown-linux-gnu && \
    cargo build --release --target aarch64-apple-darwin && \
    cargo build --release --target x86_64-apple-darwin && \
    cargo build --release --target x86_64-unknown-linux-gnu && \
    cargo build --release --target x86_64-unknown-linux-musl
