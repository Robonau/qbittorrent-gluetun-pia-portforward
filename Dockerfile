FROM rust:1.80.1-slim-bullseye AS builder

WORKDIR /app
COPY . .

RUN apt-get update -y && \
    apt-get install -y pkg-config \
    libssl-dev

RUN cargo build --release

RUN strip target/release/qbittorrent-gluetun-pia-portforward

FROM debian:bullseye-slim AS release

WORKDIR /app

COPY --from=builder /app/target/release/qbittorrent-gluetun-pia-portforward .

CMD ["./qbittorrent-gluetun-pia-portforward"]