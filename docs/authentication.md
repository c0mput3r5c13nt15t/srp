# Authentication

## Key generation

Generate an asymmetric key pair for authenticating the proxy to the gateway:

```bash
openssl genpkey -algorithm ed25519 -out sample_privkey.pem -outpubkey sample_pubkey.pem
```

The proxy will use the private key, while the gateway uses the public key. You will need a public private key pair per proxy.


## Configuration

### Proxy

Add the following line to the proxy:

```toml
private_key="./examples/keys/sample_privkey_tcp.pem"
```

### Gateway

On the gateway add a public key per proxy:

```toml
[gateway.pubkeys]
"tcp_proxy" = "./examples/keys/sample_pubkey_tcp.pem"
"udp_proxy" = "./examples/keys/sample_pubkey_udp.pem"
```
