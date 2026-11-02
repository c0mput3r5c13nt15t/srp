# Deployment View

## Overview

The deployment model is intentionally simple and self-hosted. The project separates the public relay from the private side, with the gateway deployed on a public server and the proxy deployed near the internal service.

## Typical topology

```text
External client
      |
      v
Public gateway (VPS)
      |
      | QUIC tunnel
      v
Private proxy
      |
      v
Internal service
```
## Public side

The gateway runs on a public server or VPS and is responsible for:

- listening on a public address
- accepting proxy connections
- validating requested exposure
- exposing TCP or UDP ports

## Private side

The proxy runs in the private network and is responsible for:

- establishing outbound connectivity to the gateway
- registering the service configuration
- forwarding traffic to the local app

## Deployment variants

### Single service deployment

A common deployment is:

- one public gateway
- one private proxy
- one internal application

This is the baseline model for exposing a service behind NAT.

### Multiple services

Multiple services can be exposed by running multiple proxies or by assigning multiple public ports. The architecture supports this as long as ports and exposure configuration remain explicit.

### UDP deployment

The same topology can be used for UDP services, with the gateway binding a public UDP socket and relaying datagrams to the local application.

## Configuration model

Both components are configured via TOML files. The configuration defines:

- bind addresses and ports
- gateway connection details
- local application target
- public exposure target
- protocol type

## Summary

The deployment view reflects the main design idea of the project: keep the service private, host the relay publicly, and connect both through a secure tunnel. This creates a lightweight and understandable self-hosted deployment model.
