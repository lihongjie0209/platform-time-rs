FROM rust:1-slim-bookworm AS builder
WORKDIR /app

# Cache dependencies layer
COPY Cargo.toml ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -f target/release/deps/platform_time_rs*

# Build actual source
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /app/target/release/platform-time-rs ./
EXPOSE 8080
CMD ["./platform-time-rs"]
