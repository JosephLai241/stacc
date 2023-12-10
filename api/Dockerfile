FROM rust:1.67

WORKDIR /api

COPY . .

RUN cargo build --release

EXPOSE $STACC_API_PORT_NUMBER

CMD [ "target/release/api" ]
