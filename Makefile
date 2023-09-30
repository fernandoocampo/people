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

.PHONY: add-person
add-person: ## add a person, name will be $(date)
	curl -H "Content-Type: application/json" \
	--data '{"name":"Esme"}' \
	-X POST http://localhost:3030/people

.PHONY: get-people
get-people: ## get the existing people in the service
	curl -X GET http://localhost:3030/people


