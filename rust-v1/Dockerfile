FROM rust:1.76 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-v1 .
COPY data ./data
ENV APP_VERSION=1.0.0
CMD ["./rust-v1"] 