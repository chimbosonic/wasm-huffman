FROM rust:latest

RUN apt-get update && apt-get install -y nodejs npm
RUN groupadd --gid 1000 wasm \
    && useradd --uid 1000 --gid wasm --shell /bin/bash --create-home wasm
USER wasm
RUN cargo install wasm-pack