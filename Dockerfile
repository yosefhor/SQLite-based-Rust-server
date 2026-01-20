FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /app/target/release/SQLite_based_Rust_server .
COPY migrations ./migrations
RUN mkdir -p data logs
EXPOSE 8000
CMD ["./SQLite_based_Rust_server"]