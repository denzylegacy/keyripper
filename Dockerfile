FROM rust:latest

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev

ENV OPENSSL_DIR=/usr/lib/ssl
ENV OPENSSL_LIB_DIR=/usr/lib/ssl/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include/ssl

WORKDIR /app
COPY . /app
RUN cargo build --release
