FROM rust:1.67

WORKDIR /app

COPY Cargo.* ./

RUN cargo build --release

COPY src ./src

EXPOSE $PORT_NUMBER

CMD [ "target/release/api" ]
