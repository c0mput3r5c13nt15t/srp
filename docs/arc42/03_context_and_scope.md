# Context and Scope

## Context

The project addresses a common self-hosting problem: how to expose a service running in a private network to the internet without exposing the service host directly. The system provides a secure relay architecture in which a public gateway forwards traffic to a private proxy, which then forwards it to the local application.

The system consists of:

- a public relay on an external host (for example a VPS)
- a private proxy close to the service
- a QUIC tunnel between both endpoints
- the internal application that is being exposed

## Actors and systems

### External client

An external user or system connects to the public endpoint exposed by the gateway.

### Gateway

The gateway runs on the public side. It accepts connections from proxies, validates the requested exposure, and forwards traffic to the appropriate local service.

### Proxy

The proxy runs in the private network next to the service. It connects to the gateway, registers the service exposure configuration, and forwards traffic to the internal application.

### Internal application

The actual service being exposed, such as a web service, API, or UDP-based service.

## Business and technical motivation

The project is designed for users who want to:

- expose self-hosted services without opening ports directly on the private network
- keep services running behind NAT or home-network boundaries
- avoid per-service public IP requirements
- operate a simple, self-hosted relay system

## In scope

- public relay on a VPS or other reachable host
- outbound connection from private proxy to public gateway
- encrypted communication between gateway and proxy
- TCP and UDP forwarding
- explicit IP/port mapping
- configuration-driven deployment
- transparent forwarding of traffic

## Out of scope

- application-specific routing logic
- user management or authentication
- multi-tenant cloud relay platform features
- advanced orchestration or service discovery
- protocol translation
- managed SaaS gateway functionality

## Scope boundaries

The system is a reverse-proxy and tunneling solution. It focuses on exposing private services through a relay without modifying the service itself. It is not intended to replace a full API gateway, ingress controller, or hosted cloud reverse-proxy platform.

## Summary

The project solves the problem of secure service exposure from private environments through a lightweight relay model. The scope is intentionally limited to tunnel-based forwarding and forwarding infrastructure, not to broader platform features.
