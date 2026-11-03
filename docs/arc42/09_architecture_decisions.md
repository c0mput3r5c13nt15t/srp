# Architecture Decisions

## Overview

The project’s architecture is based on a small number of deliberate decisions that focus on ensuring a secure and tansparent service exposure of a private service through the use of the proxy and gateway.

## Decision 1: split gateway and proxy responsibilities

The system is divided into `srp_gateway` and `srp_proxy`.

This separation is neccessary because:

- the public relay must be reachable from the internet
- the private proxy must connect out from the private network
- responsibilities are easier to reason about when split
- by separating these two, a 'trust boundary' is established, ensuring that no unneccessary information from the private service or proxy is exposed to the gateway or external client

## Decision 2: use QUIC as the tunnel transport

The connection between the public and private sides is based on QUIC. This gives the system a modern encrypted transport with support for reliable long-lived connections.

## Decision 3: keep the relay transparent

The relay does not rewrite application traffic or enforce application-specific logic. It forwards traffic as-is. This is a strong design choice because it makes the system compatible with many service types.

## Decision 4: keep configuration explicit

The project uses TOML configuration files to define addresses, ports, and protocol behavior. The proxy configures the gateway, so any additional configuration of the gateway is not neccessary. This reduces hidden behavior and keeps the setup understandable.

## Decision 5: use Rust

The implementation is in Rust because the project is a networking-heavy, concurrency-sensitive system. Rust provides good performance and memory safety for this kind of relay work.

## Decision 6: support both TCP and UDP

The system is not limited to HTTP-style traffic. It supports both stream-based and datagram-based services, which fits the self-hosting use case.
