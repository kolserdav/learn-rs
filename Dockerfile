FROM rust:1.67-alpine

WORKDIR /usr/src/app

COPY . .
COPY ./config/vimrc.txt /root/.vimrc

RUN cargo install --path .
