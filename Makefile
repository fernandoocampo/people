run:
	cargo run
fmt:
	cargo fmt
lint-fmt:
	cargo fmt --all --check
lint-clippy:
	cargo clippy -- -D warnings
