# syntax=docker/dockerfile:1

# -----------------------------------------------------------------------------
# Build stage — compile configra-ops CLI from workspace root
# -----------------------------------------------------------------------------
FROM rust:1.83-bookworm AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock rust-toolchain.toml deny.toml ./
COPY crates ./crates

RUN cargo build --release -p configra-ops

# -----------------------------------------------------------------------------
# Runtime stage — minimal image with health probe
# -----------------------------------------------------------------------------
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/configra-ops /usr/local/bin/configra-ops

ENV CONFIGRA_LOG_FORMAT=json \
    CONFIGRA_LOG_LEVEL=info \
    CONFIGRA_METRICS_ENABLED=true

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD configra-ops health || exit 1

ENTRYPOINT ["configra-ops"]
CMD ["health"]
