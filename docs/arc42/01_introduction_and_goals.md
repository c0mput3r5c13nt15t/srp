# Introduction and Goals
Expose self-hosted apps from your home network to the internet via an external gateway server (hosted on a VPS).

+ Expose anything from a basic HTTP(S) endpoint to a game severs using UDP
+ Fully transparent gateway server -> TLS connections only terminate at the agents
+ TLS connections only terminate at proxies
+ Easy configuration and deployment
+ Minimal resources required by relay server
+ Monitor service health
+ Build open source with self-hosting at heart

## Requirements Overview
### Functional
#### Must have
+ Proxy UDP connections
+ Proxy TCP connections
+ Proxy HTTP + HTTPS connections
+ Secure authentication of proxies to gateway
+ Logging -> configurable
#### Should have
+ Monitoring via Healthchecks
#### Could have
+ Automatic TLS certificates (letsencrypt certbot)
+ IDS/IPS integration
#### Won't have
+ Client application 

### Non-Functional
#### Must have
+ Configuration via .toml config file
+ CLI interface
+ TLS termination of HTTPS connections at proxy
+ Usage documentation (Manpage for CLI)
+ Proxying is transparent to clients
#### Should have
+ Offer pre-built GNU/Linux binaries
+ Resilience to network interruptions
+ Low resource demand
+ Network performance (jitter, latency, throughput) comparable to competitors (rathole, frp)
+ High test coverage
+ Run as systemd-Service
#### Could have
+ Resilience to (D)DOS-Attacks
+ Offer docker images
#### Won't have
+ Fancy UI


## Quality Goals
+ Security
+ Reliability
+ Performance

TODO:

A table with quality goals and concrete scenarios, ordered by priorities



## Stakeholders
+ Product Owners/Developers - Deliver a working product according to specified requirements, good grade  
+ Lecturers - Adherence to pre-established software development methodology
+ Users - A secure and easy to use product
+ Contributors - Low bar of entry, good documentation, timely responses to requests

