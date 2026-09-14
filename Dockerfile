FROM rust:1.98.1-slim-bullseye AS builder
WORKDIR /usr/src/agents

# Multi-stage Dockerfile for building a Leptos (Rust) application
RUN apt-get update && apt-get install -y --no-install-recommends \
	build-essential \
	pkg-config \
	libssl-dev \
	ca-certificates \
	&& rm -rf /var/lib/apt/lists/*

COPY common .
COPY platypus-agent-kafka .
COPY Cargo.toml .

RUN cargo build --release

FROM debian:bullseye-slim AS runtime

FROM debian:bullseye-slim

RUN apt update \
    && apt install --yes ca-certificates gettext-base libssl1.1 --no-install-recommends \
    && rm -rf /var/lib/{apt,dpkg,cache,log}

COPY --from=build "/target/release/platypus-agents" "/bin/platypus-agents"
COPY --from=build "/tls" "/etc/tls"

ENV RUST_LOG info
ENV DB_HOST "postgres://db_user:Platypus2025!@172.17.0.1/platypus"
ENV TLS_CERT "/etc/tls/cert.pem"
ENV TLS_KEY "/etc/tls/key.pem"

EXPOSE 8001

CMD ["/bin/platypus"]