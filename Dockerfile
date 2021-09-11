ARG APP=xkcdbot-rs

FROM rust:latest AS builder
ARG APP
WORKDIR /usr/src/${APP}
COPY . .
RUN cargo build --release

FROM debian:buster-slim 
ARG APP
RUN apt-get update \ 
    && apt-get install -y ca-certificates \ 
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/${APP}/target/release/${APP} /usr/local/bin/${APP}
CMD ${APP}
