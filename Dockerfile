FROM rust:1.87 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

RUN chown 65532:65532 ./target/release/flatboat-operator
RUN chmod u+x ./target/release/flatboat-operator

FROM debian:latest

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/flatboat-operator ./

ENV RUST_LOG="trace"

USER 65532:65532

CMD ["/usr/local/bin/flatboat-operator"]
