# Architecture

## Overview

Savings is a self-hosted LLM gateway. It sits between clients (the Rust CLI) and a
privately-hosted 30B coding model (served with vLLM on your own hardware), adding
caching, budgets, retrieval, compression, tool-calling, and an audit ledger — all
running for $0/month on free-tier infrastructure.

## Components

| Component | Language | Runs on | Purpose |
|---|---|---|---|
| `serving/` | — (vLLM) | Homelab GPU box | Serves the 30B model over an OpenAI-compatible API |
| `gateway/` | Python (FastAPI) | Oracle ARM VM (k3s) | Cache, budget, RAG, compression, ledger, tools |
| `cli/` | Rust | User machines | Client, `brew install`-able |
| `infra/` | Terraform | — | Provisions the Oracle VM, Tailscale, k3s |
| `deploy/` | Helm | k3s | Deploys the gateway + supporting services |
| `observability/` | YAML | k3s | Prometheus, Loki, Grafana, Alloy |

## Network

Everything communicates over **Tailscale**. Nothing is exposed to the public internet.

- CLI → Gateway: over the tailnet
- Gateway → vLLM: over the tailnet
- Gateway → Redis: in-cluster (k3s)

## Data Flow (request lifecycle)

1. CLI sends a chat request to the gateway over Tailscale.
2. Gateway checks the budget for the user/day.
3. Gateway computes a cache key; checks Redis (hot) then SQLite (cold).
4. On cache miss: augment prompt via retrieval, compress, forward to vLLM.
5. Stream tokens back to the CLI; write to cache.
6. Append the request to the tamper-evident ledger.
7. Emit metrics to Prometheus, logs to Loki.

> This document describes the target architecture. Implementation is in progress.
