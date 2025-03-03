# Build stage
FROM rust:1.82 AS builder

WORKDIR /
COPY ./src ./src
COPY ./Cargo.toml ./

RUN cargo install wasm-pack && \
  cargo build --release -p api && \
  wasm-pack build ./src/app --target web --no-typescript --release

# Prod stage
FROM debian:stable-slim

EXPOSE 8080

COPY --from=builder ./target/release/api /

COPY --from=builder ./src/app/pkg/app_bg.wasm /public/
COPY --from=builder ./src/app/pkg/app.js /public/

COPY ./public /public/

ENTRYPOINT ["./api"]
