FROM rust:1.80.1-slim-bullseye

WORKDIR /usr/src/app
COPY . .

RUN apt-get update -y && \
    apt-get install -y pkg-config \
    libssl-dev

RUN cargo install --path .

CMD ["qbittorrent-gluetun-pia-portforward"]