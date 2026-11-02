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
