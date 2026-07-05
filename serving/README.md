vLLM / Ollama serving

The gateway only speaks the OpenAI API, so any backend that exposes it works.
This directory bundles two: Ollama for local dev, vLLM for the homelab GPU box.
Both land on port 8000, so switching between them is one env var in the gateway,
not a code change.

Run one:

    docker compose --profile ollama up -d
    docker compose --profile vllm up -d

Ollama needs a model pulled after it starts:

    docker compose exec ollama ollama pull qwen2.5-coder:1.5b

Check it's serving:

    curl http://localhost:8000/v1/models
