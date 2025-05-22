# Stage di build (heavy version)
FROM rust:1.82 as builder

WORKDIR /app


RUN apt update && apt install -y ffmpeg pkg-config libssl-dev


COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY src ./src


RUN cargo build --release --target x86_64-unknown-linux-gnu

# Stage di runtime (ligher version)
FROM debian:bookworm-slim

RUN apt update && \
    apt install -y ffmpeg ca-certificates && \
    apt clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/backend .

ENV RUST_LOG=info \
    RUST_BACKTRACE=1

EXPOSE 8000

# Comando di avvio
CMD ["./backend"]
