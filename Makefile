.PHONY: help install lint serve serve-down

.DEFAULT_GOAL := help

BACKEND ?= ollama

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
