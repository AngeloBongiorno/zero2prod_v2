FROM rust:1.91.0-bookworm

WORKDIR /app

RUN apt update && apt install lld clang -y

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release

ENTRYPOINT ["./target/release/zero2prod"]
