.PHONY: run
run: ## run the application using cargo.
	RUST_LOG=debug LOG_SYSTEM=log4rs cargo run

.PHONY: fmt
fmt: ## format basecode using cargo.
	cargo fmt

.PHONY: test
test: ## run unit tests using cargo.
	cargo test

.PHONY: lint-fmt
lint-fmt: ## checkc that code format is ok.
	cargo fmt --all --check

.PHONY: lint-clippy
lint-clippy: ## apply linter clippy.
	cargo clippy  --future-incompat-report -- -D warnings

.PHONY: start-services
start-services: ## start database
	docker compose -f docker-compose.yml up

.PHONY: stop-services
stop-services: ## stop database
	docker compose down --volumes

