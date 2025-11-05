FROM rust:1.90-bookworm AS builder

ARG TUNNEL_HOST

WORKDIR /app

COPY . .

RUN cargo install cargo-make

RUN cargo make setup

RUN apt update
RUN apt-get install pkg-config libssl-dev

RUN cargo install cargo-binstall

RUN cargo binstall dioxus-cli

RUN cargo make build-web

FROM nginx:alpine

COPY --from=builder /app/target/dx/wingedcap-interactive-server/release/web/public /usr/share/nginx/html

CMD ["nginx", "-g", "daemon off;"]
