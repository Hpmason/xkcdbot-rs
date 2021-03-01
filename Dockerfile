FROM rust:latest AS builder

WORKDIR /usr/src/xkcdbot-rs
COPY . .

RUN cargo install --path .

CMD ["xkcdbot-rs"]

FROM debian:buster-slim 
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/xkcdbot-rs /usr/local/bin/xkcdbot-rs
CMD ["xkcdbot-rs"]