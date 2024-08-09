<!--
 Copyright 2024 robonau
 
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 
     https://www.apache.org/licenses/LICENSE-2.0
 
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-->

# qbittorrent gluetun PIA port forward

a simple script for [qbittorrent](https://github.com/qbittorrent/qBittorrent) meant to be used with [Gluetun](https://github.com/qdm12/gluetun).

Getting VPN port forwarding set up when using containers can be a pain since the port number is dynamic. This script automatically updates the incoming port for qbittorrent based on the current forwarded port.

## Usage

1. gluetun should create a file called forwarded_port.
2. that file should be passed to the PIA-port-forward container.
the container will exit if the environment variables are incorrect or if it cant access qbittorrent's api.

```yaml
services:
  gluetun:
    image: qmcgaw/gluetun
    ...
    environment:
      ...
      - PORT_FORWARDING=on
      - PORT_FORWARDING_STATUS_FILE=/gluetun/forwarded_port
    volumes:
      - ./gluetun:/gluetun
  qbittorrent:
    image: lscr.io/linuxserver/qbittorrent:latest
    ...
    environment:
      - WEBUI_PORT=8080
    network_mode: service:gluetun
    depends_on:
      - gluetun
  qbittorrent-gluetun-PIA-port-forward:
    image: ghcr.io/robonau/qbittorrent-gluetun-pia-portforward:latest
    environment:
      - QBITTORRENT_URL=http://gluetun:8080
      - QBITTORRENT_USERNAME=username
      - QBITTORRENT_PASSWORD=password
    volumes:
      - ./gluetun/forwarded_port:/forwarded_port:ro
    depends_on:
      - gluetun
      - qbittorrent
    restart: unless-stopped
```

3. Make sure you're using a VPN region that supports port forwarding.