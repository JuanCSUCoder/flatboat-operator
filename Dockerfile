FROM rust:1.87 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM scratch

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/flatboat-operator ./

ENV RUST_LOG="trace"

CMD ["/usr/local/bin/flatboat-operator"]
