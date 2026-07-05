# savings

savings runs a 30B coding model on hardware you already own and puts a small Python
gateway in front of it. The gateway caches repeated requests, tracks token budgets,
pulls in context from your own code, and keeps an audit log of everything it serves.
You talk to it through a CLI you install with `brew`.

Everything runs behind Tailscale on a free Oracle ARM VM, so it costs nothing to run.

> [!WARNING]
> Early days. Nothing here works yet; this is the project scaffold.

## How it works

Three machines, connected over Tailscale:

- Your homelab box serves the model with [vLLM](https://github.com/vllm-project/vllm).
- A free Oracle VM runs the gateway (FastAPI), Redis, and the observability stack on k3s.
- Your laptop runs the CLI.

The CLI sends a request to the gateway, the gateway checks its cache and budget, adds
retrieval context, forwards to the model, and streams tokens back.

## Layout

```
gateway/        Python gateway (FastAPI)
cli/            Rust CLI
serving/        vLLM config for the model host
infra/          Terraform + k3s
deploy/         Helm chart
observability/  Prometheus, Loki, Grafana
load-tests/     k6 scenarios
```

## License

MIT
