FROM rust:1.70-slim-buster

RUN apt-get update -y && apt-get upgrade -y \
    && apt-get install -y sqlite3 libsqlite3-0
RUN cargo install cargo-watch

WORKDIR /app
COPY ./ /app
#  RUN cargo build