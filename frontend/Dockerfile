FROM rust:1.67

WORKDIR /frontend

COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked trunk

CMD [ "trunk", "serve", "--release" ]
