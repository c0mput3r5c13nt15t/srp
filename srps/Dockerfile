FROM rust:1.88

WORKDIR /app
COPY . .

RUN cargo build --release -p srps

CMD ["./target/release/srps" , "-c", "/app/server.toml"]