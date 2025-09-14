BACKEND_DIR = src/todo_ic_backend

.PHONY: help build test clean start stop deploy deploy-mainnet deploy-testnet test-api test-api-testnet lint fmt

help:
	@echo "Available commands:"
	@echo "  make test"
	@echo "  make build"
	@echo "  make deploy"
	@echo "  make deploy-testnet"
	@echo "  make deploy-mainnet"
	@echo "  make test-api"
	@echo "  make test-api-testnet"
	@echo "  make clean"

build:
	@dfx build todo_ic_backend

test:
	@cd $(BACKEND_DIR) && cargo test --verbose

lint:
	@cd $(BACKEND_DIR) && cargo clippy --all-targets --all-features -- -D warnings

fmt:
	@cd $(BACKEND_DIR) && cargo fmt

start:
	@dfx start --clean --background

stop:
	@dfx stop

deploy: build
	@dfx deploy todo_ic_backend

deploy-testnet: build
	@dfx deploy todo_ic_backend --network playground

deploy-mainnet: build
	@dfx deploy todo_ic_backend --network ic

test-api:
	@dfx canister call todo_ic_backend add_todo '(record { text = "Test todo" })'
	@dfx canister call todo_ic_backend get_all_todos '(record { offset = 0; limit = 10 })'

test-api-testnet:
	@dfx canister call 25x2w-paaaa-aaaab-qackq-cai add_todo '(record { text = "Test from Makefile" })' --network playground
	@dfx canister call 25x2w-paaaa-aaaab-qackq-cai get_all_todos '(record { offset = 0; limit = 10 })' --network playground
	@dfx canister call 25x2w-paaaa-aaaab-qackq-cai get_todo_count '()' --network playground

clean:
	@cd $(BACKEND_DIR) && cargo clean
	@rm -rf .dfx/local