FROM rust:1.52.1

ENV USER=root

RUN mkdir -p /app/

WORKDIR /app/

COPY . /app/

RUN apt-get install curl

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN cd ./wasm_modules/actions_router &&  \ 
    wasm-pack build --target web --out-name actions_router --out-dir ../../app/web_layout/wasm/actions_router --no-typescript && \
    cd ../geometry && \
    wasm-pack build --target web --out-name geometry --out-dir ../../app/web_layout/wasm/geometry --no-typescript && \
    cd ../properties && \
    wasm-pack build --target web --out-name properties --out-dir ../../app/web_layout/wasm/properties --no-typescript && \
    cd ../renderer && \
    wasm-pack build --target web --out-name renderer --out-dir ../../app/web_layout/wasm/renderer --no-typescript

EXPOSE 8080
