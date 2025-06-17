FROM rust:1.76 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-v2 .
COPY data ./data
ENV APP_VERSION=2.0.0
CMD ["./rust-v2"] 