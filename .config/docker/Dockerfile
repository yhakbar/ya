FROM rust:latest

RUN rustup default nightly && \
    rustup update

RUN rustup target add aarch64-unknown-linux-gnu

WORKDIR /app
ENTRYPOINT [ "bash"]
