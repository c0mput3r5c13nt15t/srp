FROM rust:1.88

WORKDIR /app
COPY . .

RUN cargo build --release -p srp_proxy

CMD ["./target/release/srp_proxy" , "-c", "/app/proxy.toml"]
