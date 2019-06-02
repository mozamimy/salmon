FROM rust:1.35-stretch as builder

RUN mkdir -p /build/src
RUN echo 'fn main() {}' > /build/src/main.rs
COPY Cargo.toml Cargo.lock /build/
WORKDIR /build/
RUN cargo build --release --locked
COPY src /build/src
RUN find src -type f -exec touch {} +  && cargo build --release --locked

FROM debian:stretch-slim
COPY --from=builder /build/target/release/salmon /usr/local/sbin/

CMD ["/usr/local/sbin/salmon", "--help"]
