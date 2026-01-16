# Dockerfile pour SMP Moules Export Fiches
# Optimisé pour Railway

FROM rust:1.75-slim-bookworm as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/smp_contact_export /app/server

ENV RUST_LOG=info
ENV HOST=0.0.0.0

EXPOSE 8080

CMD ["./server"]
