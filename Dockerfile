# Web APIサーバ用のビルド
FROM rust:1.69-slim-buster as server

WORKDIR /usr/src/myapp
RUN apt-get update && \
    apt install -y libpq-dev &&\
    cargo install diesel_cli --no-default-features --features postgres

COPY . .

# E2Eテストを実行するクライアント用のビルド
FROM rust:1.69-slim-buster as client

WORKDIR /usr/src/myapp

RUN apt-get update && \
    apt install -y wait-for-it

COPY e2e/ .