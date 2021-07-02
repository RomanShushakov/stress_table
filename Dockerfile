FROM python:3.8.5

RUN mkdir -p /app/

WORKDIR /app/

COPY ./secrets/app_secrets.py /app/

ENTRYPOINT ["python", "./app_secrets.py"]

FROM rust:1.53

ENV USER=root

RUN mkdir -p /app/

WORKDIR /app/

COPY . /app/

RUN apt-get install curl

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN cd ./wasm_modules/actions_router &&  \ 
    wasm-pack build --target web --out-name actions_router --out-dir ../../app/web_layout/wasm/actions_router --no-typescript && \
    cd ../fe_model && \
    wasm-pack build --target web --out-name fe_model --out-dir ../../app/web_layout/wasm/fe_model --no-typescript && \
    cd ../renderer && \
    wasm-pack build --target web --out-name renderer --out-dir ../../app/web_layout/wasm/renderer --no-typescript

EXPOSE 8080
