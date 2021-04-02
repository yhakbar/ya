# This doesn't currently work. I'm not sure why.

# I think it's probably my host machine being an M1 resulting
# in cross compilation being a problem.

FROM rust:latest

RUN rustup default nightly && \
    rustup target add aarch64-apple-darwin && \
    rustup target add aarch64-unknown-linux-gnu && \
    rustup target add x86_64-apple-darwin && \
    rustup target add x86_64-unknown-linux-gnu && \
    rustup target add x86_64-unknown-linux-musl
