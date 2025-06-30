FROM rust:1.88 AS builder

WORKDIR /usr/src/weight-tracker

COPY .sqlx .sqlx
COPY migrations migrations
COPY src src
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

ENV SQLX_OFFLINE=true
RUN cargo build --release --bin weight-tracker

FROM debian:bookworm-slim

RUN mkdir -p /etc/weight-tracker

COPY static static
COPY templates templates
COPY --from=builder /usr/src/weight-tracker/target/release/weight-tracker /usr/local/bin/weight-tracker

ENV DATABASE_URL=sqlite:///etc/weight-tracker/weight-tracker.db
ENV LISTEN_ADDRESS=0.0.0.0
CMD ["weight-tracker"]
