.PHONY: help install lint serve serve-down serve-pull serve-models

.DEFAULT_GOAL := help

BACKEND ?= ollama
MODEL ?= qwen2.5-coder:1.5b

help:
	@grep -Eh '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "%-12s %s\n", $$1, $$2}'

install:
	pre-commit install

lint:
	pre-commit run --all-files

serve:
	docker compose --env-file .env -f serving/docker-compose.yml --profile $(BACKEND) up -d

serve-down:
	docker compose -f serving/docker-compose.yml --profile $(BACKEND) down

serve-pull:
	docker compose -f serving/docker-compose.yml exec -T ollama ollama pull $(MODEL)

serve-models:
	@docker compose -f serving/docker-compose.yml exec -T ollama ollama list
