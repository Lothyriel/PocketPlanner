# Build stage
FROM rust:1.82 AS builder

WORKDIR /
COPY ./src ./src
COPY Cargo.toml ./
COPY build.sh ./

RUN cargo install wasm-pack
RUN chmod +x ./build.sh && ./build.sh 

# Prod stage
FROM debian:stable-slim

EXPOSE 8080

COPY --from=builder /target/release/api /
COPY /public /public

ENTRYPOINT ["./api"]
