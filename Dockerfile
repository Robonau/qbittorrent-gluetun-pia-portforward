FROM rust:1.80.1-slim-bullseye

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["qbittorrent-gluetun-pia-portforward"]