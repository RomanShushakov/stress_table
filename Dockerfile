FROM rust:1.50

ENV USER=root

RUN mkdir -p /app/

WORKDIR /app/

COPY . /app/

RUN apt-get install curl

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

EXPOSE 8080