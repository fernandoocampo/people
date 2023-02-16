.PHONY: run
run: ## run the application using cargo.
	cargo run

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
	cargo clippy -- -D warnings
