FROM rust:latest

WORKDIR /usr/src/xkcdbot-rs
COPY . .

RUN cargo install --path .

CMD ["xkcdbot-rs"]

