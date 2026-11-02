# Runtime View

## Overview

The most relevant runtime behavior is the establishment of a QUIC connection between the private proxy and the public gateway, followed by transparent forwarding to the internal application.

## Proxy registration and connection setup

1. The proxy reads its configuration.
2. It connects to the gateway via QUIC.
3. It sends a config request describing the service to be exposed.
4. The gateway validates the request.
5. If accepted, the gateway binds the public socket and starts forwarding.
6. If rejected, the proxy stops with an error.

This creates the runtime contract between gateway and proxy.

## TCP flow

For TCP services:

1. the proxy connects to the gateway
2. the gateway binds a public TCP listener
3. traffic from the external client is forwarded to the proxy
4. the proxy forwards it to the local application
5. responses are returned along the same path

This is suitable for typical web services and APIs.

## UDP flow

For UDP services:

1. the gateway binds a public UDP socket
2. the proxy forwards incoming datagrams to the local service
3. responses are relayed back through the same tunnel path

This supports datagram-based services such as game servers or custom UDP protocols.

## Shutdown flow

Both gateway and proxy handle shutdown using cancellation tokens. They stop new work, close active connections, and finish cleanly before exiting.

## Summary

At runtime, the system behaves like a secure tunnel with a simple control flow: connect, validate, expose, and forward. The core runtime behavior is transparent forwarding of service traffic across the public/private boundary.
