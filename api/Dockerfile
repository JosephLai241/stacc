FROM rust:latest

WORKDIR /api

COPY . .

RUN cargo build --release

ARG STACC_API_PORT_NUMBER

EXPOSE $STACC_API_PORT_NUMBER

CMD [ "target/release/api" ]
