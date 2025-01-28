FROM rust:1.84 AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path . --root /usr/local/bin/app

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/app /usr/local/bin/app
CMD ["/usr/local/bin/app/bin/dawarkitty"]
