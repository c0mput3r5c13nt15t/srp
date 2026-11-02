# Architecture Constraints

## Overview

The architecture is shaped by the requirement to expose self-hosted services securely through a public relay without requiring the service itself to be directly reachable from the internet. This leads to a simple public/private split: a relay on a public VPS and a proxy next to the service in the private network.

## Technical constraints

- The project is implemented as a Rust workspace with a clear separation between:
  - `srp_gateway`
  - `srp_proxy`
  - `shared`
- The public relay must stay lightweight and resource-efficient.
- The system must support both TCP and UDP traffic.
- Service exposure must be transparent: the target service should behave as if it were directly reachable.
- Communication between gateway and proxy is based on QUIC, using encrypted tunnel-based transport.
- Ports and exposed endpoints are configured explicitly rather than discovered dynamically.

## Security constraints

- The public relay must not become an application-layer endpoint for the internal service.
- Traffic between gateway and proxy must be encrypted.
- The internal service stays behind a private network and is not exposed directly.
- The architecture must support self-hosted operation without central platform trust assumptions.

## Operational constraints

- The private side may be behind NAT, firewall rules, or home-network restrictions.
- The gateway must be able to accept multiple proxy connections and expose services through configured ports.
- The deployment model should remain simple enough for a self-hosted environment.
- Port availability must be validated before exposure is activated.

## Design constraints

- The system follows a clear responsibility split:
  - gateway: public relay and forwarding
  - proxy: private-side connector to the service
  - shared: common contracts and configuration
- The design favors explicit configuration over complex dynamic service discovery.
- The solution is intentionally focused on transparent service exposure rather than general-purpose application routing.
