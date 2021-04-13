FROM rust:latest

RUN cargo install sd fd-find && \
    rustup default nightly && \
    rustup update && \
    rustup target add aarch64-unknown-linux-gnu

WORKDIR /app

COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo install --path .

ENTRYPOINT [ "bash"]

WORKDIR /app
