
# Use the nightly Rust image to support Edition 2024
FROM rustlang/rust:nightly as builder
WORKDIR /app
COPY . .
# We use the nightly compiler to handle the new edition
RUN cargo +nightly build --release

# Use a tiny runtime image to keep it fast
FROM debian:bookworm-slim
WORKDIR /usr/local/bin
# Note: Ensure your binary name in Cargo.toml matches 'battery-ingestor'
COPY --from=builder /app/target/release/battery-ingestor .
EXPOSE 8080
CMD ["./battery-ingestor"]