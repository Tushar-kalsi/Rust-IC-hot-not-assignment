BACKEND_DIR = src/todo_ic_backend
FRONTEND_DIR = src/todo_ic_frontend

.PHONY: help build build-backend build-frontend test clean start stop deploy deploy-mainnet deploy-testnet test-api test-api-testnet lint fmt serve-frontend install-frontend-deps

help:
	@echo "Available commands:"
	@echo "Setup:"
	@echo "  make setup-local (all-in-one: stop, start, create, build, deploy)"
	@echo "  make setup-testnet (all-in-one: check cycles, create, build, deploy)"
	@echo "Backend:"
	@echo "  make build-backend"
	@echo "  make test"
	@echo "  make deploy (local)"
	@echo "  make deploy-mainnet"
	@echo "Frontend:"
	@echo "  make build-frontend"
	@echo "  make serve-frontend"
	@echo "  make install-frontend-deps"
	@echo "Testing:"
	@echo "  make test-api (local)"
	@echo "  make test-api-testnet"
	@echo "General:"
	@echo "  make build (builds both)"
	@echo "  make clean"
	@echo "  make start (start local replica)"
	@echo "  make stop (stop local replica)"

serve: setup-local serve-frontend

build-backend:
	@dfx build todo_ic_backend

build-frontend: build-backend install-frontend-deps
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

setup-local: stop
	@echo "Setting up local development environment..."
	@echo "1. Stopping any existing DFX processes..."
	@pkill -f dfx 2>/dev/null || true
	@echo "2. Starting DFX local replica..."
	@dfx start --clean --background
	@echo "3. Creating local canister..."
	@dfx canister create todo_ic_backend
	@echo "4. Building backend..."
	@dfx build todo_ic_backend
	@echo "5. Deploying canister..."
	@dfx deploy todo_ic_backend
	@echo "✅ Local development environment ready!"
	@echo "Canister ID: $$(dfx canister id todo_ic_backend)"
	@echo "Candid UI: http://127.0.0.1:4943/?canisterId=$$(dfx canister id __Candid_UI)&id=$$(dfx canister id todo_ic_backend)"

setup-testnet:
	@echo "Setting up testnet development environment..."
	@echo "1. Checking cycles balance..."
	@dfx cycles balance --network playground || echo "No cycles found"
	@echo "2. Creating testnet canister..."
	@dfx canister create todo_ic_backend --network playground
	@echo "3. Building backend..."
	@dfx build todo_ic_backend
	@echo "4. Deploying to testnet..."
	@dfx deploy todo_ic_backend --network playground
	@echo "✅ Testnet development environment ready!"
	@echo "Canister ID: $$(dfx canister id todo_ic_backend --network playground)"
	@echo "Direct URL: https://$$(dfx canister id todo_ic_backend --network playground).icp0.io"
	@echo "Candid UI: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=$$(dfx canister id todo_ic_backend --network playground)"

start:
	@dfx start --clean --background

stop:
	@dfx stop

deploy: build-backend
	@dfx deploy todo_ic_backend


deploy-mainnet: build-backend
	@echo "Deploying to mainnet..."
	@dfx deploy todo_ic_backend --network ic
	@echo "Backend deployed to mainnet!"

deploy-mainnet-cycles: build-backend
	@echo "Deploying to mainnet with cycles..."
	@echo "WARNING: This will deploy to the live Internet Computer mainnet and consume cycles!"
	@read -p "Are you sure you want to continue? (y/N): " confirm && [ "$$confirm" = "y" ] || exit 1
	@dfx deploy todo_ic_backend --network ic --with-cycles 1000000000000
	@echo "Backend deployed to mainnet with cycles!"
	@dfx canister id todo_ic_backend --network ic

test-api:
	@dfx canister call todo_ic_backend add_todo '(record { text = "Test todo" })'
	@dfx canister call todo_ic_backend get_all_todos '(record { offset = 0; limit = 10 })'

test-api-testnet:
	@echo "Testing API on testnet..."
	@CANISTER_ID=$$(dfx canister id todo_ic_backend --network playground 2>/dev/null || echo ""); \
	if [ -z "$$CANISTER_ID" ]; then \
		echo "Error: Canister not found on testnet. Run 'make setup-testnet' first."; \
		exit 1; \
	fi; \
	echo "Using canister ID: $$CANISTER_ID"; \
	dfx canister call $$CANISTER_ID add_todo '(record { text = "Test from Makefile" })' --network playground; \
	dfx canister call $$CANISTER_ID get_all_todos '(record { offset = 0; limit = 10 })' --network playground; \
	dfx canister call $$CANISTER_ID get_todo_count '()' --network playground

clean:
	@echo "Cleaning backend..."
	@cd $(BACKEND_DIR) && cargo clean
	@echo "Cleaning frontend..."
	@cd $(FRONTEND_DIR) && cargo clean 2>/dev/null || true
	@rm -rf $(FRONTEND_DIR)/dist 2>/dev/null || true
	@echo "Cleaning DFX..."
	@rm -rf .dfx/local