# Build stage
FROM rust:1.76 as builder

COPY ./src ./src
COPY Cargo.toml ./

RUN cargo build --release

# Prod stage
FROM gcr.io/distroless/cc

EXPOSE 3000

COPY --from=builder /target/release/pocket_planner /

ENTRYPOINT ["./pocket_planner"]
