.PHONY: help install lint

.DEFAULT_GOAL := help

help:
	@grep -Eh '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "%-12s %s\n", $$1, $$2}'

install:
	pre-commit install

lint:
	pre-commit run --all-files
