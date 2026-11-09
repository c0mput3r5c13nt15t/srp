How to generate an asymmetric keypair for authenticating the proxy to the gateway:
```Bash
~$ openssl genpkey -algorithm ed25519 -out sample_privkey.pem -outpubkey sample_pubkey.pem
```