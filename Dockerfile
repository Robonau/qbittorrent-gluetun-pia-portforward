FROM rust:1.80.1-slim-bookworm AS builder

WORKDIR /app
COPY . .

RUN apt-get update -y && \
    apt-get install -y pkg-config \
    libssl-dev

RUN cargo build --release

RUN strip target/release/qbittorrent-gluetun-pia-portforward

FROM debian:bookworm-slim AS release

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libssl3 \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/*


COPY --from=builder /app/target/release/qbittorrent-gluetun-pia-portforward .

CMD ["./qbittorrent-gluetun-pia-portforward"]
