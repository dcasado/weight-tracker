FROM rust:1.79 AS builder

WORKDIR /usr/src/weight-tracker

COPY .sqlx .sqlx
COPY migrations migrations
COPY src src
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

ENV SQLX_OFFLINE=true
RUN cargo build --release --bin weight-tracker

FROM debian:bookworm-slim

COPY static static
COPY templates templates
COPY --from=builder /usr/src/weight-tracker/target/release/weight-tracker /usr/local/bin/weight-tracker

ENV LISTEN_ADDRESS=0.0.0.0
CMD ["weight-tracker"]
