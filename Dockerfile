# syntax=docker/dockerfile:1.6
# Multi-stage Dockerfile for a small production image

FROM rust:1-alpine3.20 AS builder
ARG TARGETPLATFORM
WORKDIR /app

# System deps for building native crates (e.g., rdkafka)
RUN --network=host apk add --no-cache \
        build-base \
        cmake \
        pkgconf \
        cyrus-sasl-dev \
        openssl-dev \
        zlib-dev \
        zlib-static \
        zstd-dev \
        lz4-dev

# Ensure musl targets are available for cross-builds
RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl

# Cache deps
COPY Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
# Determine musl target per Docker platform and warm cargo cache
RUN TARGET=$(case "$TARGETPLATFORM" in \
              "linux/amd64") echo x86_64-unknown-linux-musl ;; \
              "linux/arm64") echo aarch64-unknown-linux-musl ;; \
              *) echo "Unsupported TARGETPLATFORM: $TARGETPLATFORM" && exit 1 ;; \
            esac) \
    && cargo build --release --target "$TARGET" \
    && rm -rf src

# Build
COPY . .
# Build actual binary and stage it in a stable location
RUN TARGET=$(case "$TARGETPLATFORM" in \
              "linux/amd64") echo x86_64-unknown-linux-musl ;; \
              "linux/arm64") echo aarch64-unknown-linux-musl ;; \
              *) echo "Unsupported TARGETPLATFORM: $TARGETPLATFORM" && exit 1 ;; \
            esac) \
    && rm -rf target \
    && cargo build --release --target "$TARGET" \
    && mkdir -p /app/bin \
    && cp target/"$TARGET"/release/rust-backend /app/bin/app

FROM alpine:3.20 AS runtime
ARG TARGETPLATFORM
RUN --network=host apk add --no-cache \
        ca-certificates \
        cyrus-sasl \
        openssl \
        zlib \
        zstd-libs \
        lz4-libs \
        libstdc++
WORKDIR /app
COPY --from=builder /app/bin/app /usr/local/bin/app
ENV RUST_LOG=info
# Help some TLS stacks in containers
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
EXPOSE 8080
CMD ["/usr/local/bin/app"]


