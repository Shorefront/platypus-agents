FROM rust:1.98.1-bookworm as frontend-builder
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
