ARG APP=xkcdbot-rs
FROM rust:latest AS builder

WORKDIR /usr/src/${APP}
COPY . .

RUN cargo install --path .

FROM debian:buster-slim 
RUN apt-get update \ 
    && apt-get install -y ca-certificates \ 
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/${APP} /usr/local/bin/${APP}
CMD ["xkcdbot-rs"]