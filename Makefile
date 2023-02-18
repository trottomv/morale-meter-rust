.DEFAULT_GOAL := help

.PHONY: check
check:  ## Check app configuration
	cargo check
	cargo audit

.PHONY: run
fix:  ## Fix formatting and linting
	cargo fmt

.PHONY: help
help:
	@echo "[Help] Makefile list commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
