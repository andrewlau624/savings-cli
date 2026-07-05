# Savings — $0 Self-Hosted LLM Gateway

> Streams tokens from a 30B-parameter coding model on hardware you own.
> Monthly bill: **$0**. That's the whole goddamn sales pitch.

## The Pitch

| Feature | Cost |
|---|---|
| 30B coding model (private, uncensored) | $0 |
| Shared cache across users (Redis + SQLite) | $0 |
| Token budgets & rate limits | $0 |
| RAG over your private codebase | $0 |
| Prompt compression before inference | $0 |
| Rust CLI your friends can `brew install` | $0 |
| Tamper-evident request ledger | $0 |
| Prometheus + Loki + Grafana on free infra | $0 |
| Discord pages when it breaks | $0 |
| Runs on a free Oracle ARM VM behind Tailscale | $0 |
| k3s + Terraform (real infra, no bill) | $0 |
| Daily backups with a tested restore drill | $0 |

**Total per month: $0.00**

## Architecture

```
┌──────────────┐     tailscale      ┌─────────────────────────────┐
│  Your Laptop │  ──────────────►   │  Oracle Always-Free VM      │
│  (Rust CLI)  │                    │  ┌───────────────────────┐  │
└──────────────┘                    │  │  Python Gateway       │  │
                                    │  │  (FastAPI)            │  │
┌──────────────┐     tailscale      │  │  ┌─────┐ ┌─────────┐ │  │
│  Friend's    │  ──────────────►   │  │  │Cache│ │ Budget  │ │  │
│  Laptop      │                    │  │  │Redis│ │ SQLite  │ │  │
└──────────────┘                    │  │  └─────┘ │ Ledger  │ │  │
                                    │  │          └─────────┘ │  │
┌──────────────┐     tailscale      │  │  ┌──────────┐        │  │
│  Homelab     │ ◄──────────────   │  │  │ Retrieval│        │  │
│  (vLLM/GPU)  │     tailscale      │  │  │ (RAG)    │        │  │
└──────────────┘                    │  │  └──────────┘        │  │
                                    │  └───────────────────────┘  │
                                    └─────────────────────────────┘
```

## Quickstart

```bash
# Not ready yet — everything below is placeholder structure
```

## Repository Layout

```
├── gateway/        # Python FastAPI gateway service
├── cli/            # Rust CLI binary
├── serving/        # vLLM serving on your homelab device
├── infra/          # Terraform for Oracle VM + k3s provisioning
├── deploy/         # Helm chart for k3s
├── observability/  # Prometheus, Loki, Grafana, Alloy configs
├── load-tests/     # k6 test scenarios
├── docs/           # Architecture & runbooks
└── scripts/        # Backup, restore, dev helpers
```

## License

MIT# savings-cli
