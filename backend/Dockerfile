# Use the Rust base image
FROM rust:1.82

# Set the working directory
WORKDIR /usr/src/app

# Install sqlx-cli and cargo-watch tools
RUN cargo install sqlx-cli --version 0.8.2 --no-default-features --features postgres && \
    cargo install cargo-watch

# Copy dependency files and build dependencies only
COPY Cargo.toml Cargo.lock settings.json ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main(){}" > src/main.rs && \
    cargo build && \
    rm src/main.rs && \
    rm -rf target/debug/deps/backend*

# Set environment variables for better error messages
ENV RUST_BACKTRACE=1

# Use cargo-watch for hot reloading
CMD ["cargo", "watch", "-x", "run"]
