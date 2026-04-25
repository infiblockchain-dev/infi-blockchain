FROM rust:1-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --release -p infi-node

FROM debian:bookworm-slim

RUN useradd --create-home --uid 10001 infi
COPY --from=builder /app/target/release/infi-node /usr/local/bin/infi-node
RUN mkdir -p /home/infi/infi-data && chown -R infi:infi /home/infi

WORKDIR /home/infi
ENV INFI_DATA_DIR=/home/infi/infi-data

EXPOSE 8545

USER infi
CMD ["infi-node"]
