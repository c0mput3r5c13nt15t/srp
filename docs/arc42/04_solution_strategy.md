# Solution Strategy

## Overview

The solution follows a simple reverse-proxy architecture:

- a public relay (`srp_gateway`) listens on the internet-facing side
- a private proxy (`srp_proxy`) runs next to the internal service
- both communicate over a QUIC tunnel
- traffic is forwarded transparently to the application behind the proxy

This keeps the public relay focused on forwarding and connectivity, while the private network remains responsible for the local service.

## Strategy

### Public/private separation

The system is split into two runtime roles:

- `srp_gateway`: public-facing relay; validates configurations and exposes ports
- `srp_proxy`: private-side connector; registers the service and forwards traffic

This separation matches the typical self-hosting scenario where the service is behind NAT or private network boundaries.

### QUIC as transport

The tunnel uses QUIC as the underlying channel between gateway and proxy. This provides encrypted communication and fits well with long-lived relay connections.

### Support for both TCP and UDP

The project is designed to expose more than just HTTP-based services. It supports both TCP and UDP by using protocol-specific forwarding logic.

### Explicit configuration

The system relies on explicit configuration rather than dynamic discovery. Each proxy declares:

- gateway address and port
- target application address and port
- public exposure address and port
- protocol

This keeps the architecture understandable and easy to debug.

### Simple deployment

The design aims for self-hosted deployment with low operational overhead:

- binary or containerized deployment
- config files instead of a platform abstraction
- minimal infrastructure assumptions

## Summary

The chosen strategy is to keep the architecture small, explicit, and transparent: a relay on the public side, a proxy on the private side, and a secure tunnel connecting both. This is the core design decision that enables self-hosted service exposure without direct internet exposure of the service itself.
