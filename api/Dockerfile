# Build stage
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin api

ADD https://github.com/benbjohnson/litestream/releases/download/v0.3.8/litestream-v0.3.8-linux-amd64-static.tar.gz /tmp/litestream.tar.gz
RUN tar -C /usr/local/bin -xzf /tmp/litestream.tar.gz

FROM debian:stable-slim AS runtime

RUN apt-get update && apt-get install --no-install-recommends -y ca-certificates fuse3 sqlite3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/api /usr/local/bin

COPY --from=flyio/litefs:0.5 /usr/local/bin/litefs /usr/local/bin/litefs
COPY --from=builder /app/litefs.yml /etc/litefs.yml

COPY --from=builder /usr/local/bin/litestream /usr/local/bin/litestream
COPY --from=builder /app/litestream.yml /etc/litestream.yml

EXPOSE 8080
ENTRYPOINT ["litefs", "mount"]
