FROM rust:latest AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY Cargo.lock* ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /app/backend

RUN mkdir -p /app/data && chmod 777 /app/data

ENV PORT=8080
ENV HOST=0.0.0.0
ENV DATABASE_URL=sqlite:./data/todos.db
ENV RUST_LOG=info

EXPOSE 8080

RUN echo '#!/bin/sh\nset -e\necho "=== Entrypoint Script ==="\necho "Creating data directory..."\nmkdir -p /app/data\necho "Setting permissions..."\nchmod -R 777 /app/data\necho "Checking directory..."\nls -la /app/data\necho "Testing write access..."\ntouch /app/data/test.txt && rm /app/data/test.txt && echo "Write test successful"\necho "Starting backend..."\nexec /app/backend "$@"' > /app/entrypoint.sh && \
    chmod +x /app/entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]

