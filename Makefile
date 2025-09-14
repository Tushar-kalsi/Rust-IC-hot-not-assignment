BACKEND_DIR = src/todo_ic_backend
FRONTEND_DIR = src/todo_ic_frontend

.PHONY: help build build-backend build-frontend test clean start stop deploy deploy-mainnet deploy-testnet test-api test-api-testnet lint fmt serve-frontend install-frontend-deps

help:
	@echo "Available commands:"
	@echo "Backend:"
	@echo "  make build-backend"
	@echo "  make test"
	@echo "  make deploy"
	@echo "  make deploy-testnet"
	@echo "  make deploy-mainnet"
	@echo "Frontend:"
	@echo "  make build-frontend"
	@echo "  make serve-frontend"
	@echo "  make install-frontend-deps"
	@echo "General:"
	@echo "  make build (builds both)"
	@echo "  make clean"

build: build-backend build-frontend

build-backend:
	@dfx build todo_ic_backend

build-frontend: install-frontend-deps
	@echo "Building Leptos frontend..."
	@cd $(FRONTEND_DIR) && trunk build --release
	@echo "Frontend build completed!"

install-frontend-deps:
	@echo "Installing frontend dependencies..."
	@if ! command -v trunk >/dev/null 2>&1; then \
		echo "Installing trunk..."; \
		cargo install trunk; \
	fi
	@rustup target add wasm32-unknown-unknown 2>/dev/null || true

serve-frontend: install-frontend-deps
	@echo "Starting frontend development server..."
	@cd $(FRONTEND_DIR) && trunk serve --open

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

deploy: build-backend
	@dfx deploy todo_ic_backend

deploy-testnet: build-backend
	@dfx deploy todo_ic_backend --network playground

deploy-mainnet: build-backend
	@dfx deploy todo_ic_backend --network ic

test-api:
	@dfx canister call todo_ic_backend add_todo '(record { text = "Test todo" })'
	@dfx canister call todo_ic_backend get_all_todos '(record { offset = 0; limit = 10 })'

test-api-testnet:
	@dfx canister call 25x2w-paaaa-aaaab-qackq-cai add_todo '(record { text = "Test from Makefile" })' --network playground
	@dfx canister call 25x2w-paaaa-aaaab-qackq-cai get_all_todos '(record { offset = 0; limit = 10 })' --network playground
	@dfx canister call 25x2w-paaaa-aaaab-qackq-cai get_todo_count '()' --network playground

clean:
	@echo "Cleaning backend..."
	@cd $(BACKEND_DIR) && cargo clean
	@echo "Cleaning frontend..."
	@cd $(FRONTEND_DIR) && cargo clean 2>/dev/null || true
	@rm -rf $(FRONTEND_DIR)/dist 2>/dev/null || true
	@echo "Cleaning DFX..."
	@rm -rf .dfx/local