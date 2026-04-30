FROM rust:1.88

WORKDIR /app
COPY . .

RUN cargo build --release -p srp

CMD ["./target/release/srp" , "-c", "/app/client.toml"]
