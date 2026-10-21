FROM rust:1.88

WORKDIR /app
COPY . .

RUN cargo build --release -p srp_gateway

CMD ["./target/release/srp_gateway" , "-c", "/app/gateway.toml"]