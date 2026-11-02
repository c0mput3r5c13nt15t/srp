# Glossary

## Gateway

The public-side relay component (`srp_gateway`) that accepts proxy connections and exposes services through configured ports.

## Proxy

The private-side component (`srp_proxy`) that connects to the gateway and forwards traffic to the local application.

## Reverse proxy

A pattern in which traffic arriving at a public endpoint is forwarded to a private service without exposing that service directly.

## QUIC

A modern transport protocol used for encrypted and low-latency communication between gateway and proxy.

## TCP

Transmission Control Protocol, used for connection-oriented stream-based communication.

## UDP

User Datagram Protocol, used for datagram-based communication.

## Tunnel

A secure communication path between a public relay and a private application bridge.

## Public relay

The externally reachable infrastructure used to connect internet clients to private services.

## Private service

The service running in a local or private network and not directly exposed to the internet.
