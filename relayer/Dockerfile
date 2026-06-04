# syntax=docker/dockerfile:1
#
# Multi-stage build for the fast-fill relayer (a no-port, outbound-only worker).
# Builds natively for whatever arch you build on — amd64 on Fly, arm64 on a Raspberry Pi —
# so there is no cross-compilation to wrestle with. Build context is this `relayer/` dir:
#
#   docker build -t fastfill-relayer relayer/
#
# Uses cargo-chef + BuildKit cache mounts so dependencies (alloy is large) are compiled once
# and reused across rebuilds — only your own source recompiles after the first build.

# ── chef: toolchain + cargo-chef (this layer is cached until the base image changes) ──────────
FROM rust:1-bookworm AS chef
WORKDIR /app
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install cargo-chef --locked

# ── planner: distill the dependency graph into a recipe (changes only when deps change) ───────
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ── builder: cook deps from the recipe, then build the bot binary ─────────────────────────────
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Compile dependencies only. Cached in the BuildKit mounts below, so this is skipped on
# rebuilds where Cargo.lock / Cargo.toml are unchanged.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --recipe-path recipe.json
COPY . .
# Build the relayer, then copy it out of the cached `target/` mount so it persists in the layer.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked --bin fastfill-relayer && \
    cp target/release/fastfill-relayer /app/fastfill-relayer

# ── runtime: slim image with just the TLS deps the binary links against ───────────────────────
FROM debian:bookworm-slim AS runtime
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*
# Run as a non-root user (the relayer needs no special privileges).
RUN useradd --create-home --uid 10001 relayer
USER relayer
WORKDIR /home/relayer
COPY --from=builder /app/fastfill-relayer /usr/local/bin/fastfill-relayer

# Default log level (the binary already defaults to `info`; set here so it is explicit/overridable).
ENV RUST_LOG=info

# Config is supplied via env vars at run time: at minimum ALCHEMY_API_KEY, plus RELAYER_PRIVATE_KEY
# for live mode. Set RELAYER_DRY_RUN=1 (or append `--dry-run`) for a no-tx first boot.
ENTRYPOINT ["fastfill-relayer"]
