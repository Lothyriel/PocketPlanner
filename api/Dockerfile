# Build stage
FROM rust:1.88 AS builder

WORKDIR /

# RUN apt-get update && \
#  apt-get install -y --no-install-recommends clang=1:14.0-55.7~deb12u1

# RUN git clone https://github.com/rustwasm/wasm-pack && \
#  cargo install --path ./wasm-pack

COPY ./src ./src
COPY ./Cargo.toml ./

RUN cargo build --release -p api
# && wasm-pack build ./src/app --target web --no-typescript --profile bin-size

# Prod stage
FROM debian:stable-slim

EXPOSE 8080

COPY --from=builder ./target/release/api /

# COPY --from=builder ./src/app/pkg/app_bg.wasm /public/
# COPY --from=builder ./src/app/pkg/app.js /public/

COPY ./public /public/

ENTRYPOINT ["./api"]
