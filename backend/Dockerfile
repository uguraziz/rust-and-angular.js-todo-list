FROM rust:latest AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/backend /app/backend

RUN mkdir -p /app/data && \
    chmod 755 /app/data && \
    chown -R 1000:1000 /app/data

ENV HOST=0.0.0.0
ENV DATABASE_URL=sqlite:./data/todos.db?mode=rwc

EXPOSE 8080

CMD ["./backend"]

