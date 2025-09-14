# Todo IC

A simple todo list canister for the Internet Computer built with Rust.

## Quick Start

### Local Development
Get your local development environment ready with one command:

```bash
make setup-local
```

This command will:
1. Stop any existing DFX processes
2. Start DFX local replica with clean state
3. Create local canister
4. Build the backend
5. Deploy the canister

### Testnet Deployment
Deploy to IC Playground testnet with one command:

```bash
make setup-testnet
```

This command will:
1. Check your cycles balance
2. Create testnet canister
3. Build the backend
4. Deploy to testnet

**Note:** You need cycles from the [DFINITY faucet](https://faucet.dfinity.org/) before running testnet setup.

Your canister will be ready for testing with API endpoints available!

## Project Structure

```
todo_ic/
├── Makefile                 # Build and deployment automation
├── dfx.json                 # DFX configuration
├── Cargo.toml               # Workspace configuration
└── src/
    ├── todo_ic_backend/     # Backend canister
    │   ├── Cargo.toml       # Backend dependencies
    │   ├── todo_ic_backend.did  # Candid interface definition
    │   ├── src/
    │   │   ├── lib.rs       # Main canister entry points
    │   │   ├── types.rs     # Data structures and type definitions
    │   │   ├── storage.rs   # Stable storage implementation
    │   │   └── service.rs   # Business logic and validation
    │   └── tests/
    │       └── integration_test.rs  # Comprehensive test suite
    └── todo_ic_frontend/    # Frontend placeholder
        └── src/
            └── main.rs      # Simple frontend stub
```

### Key Components

#### Backend (`src/todo_ic_backend/`)

- **lib.rs**: Main canister file that defines the public API endpoints using IC CDK macros (`#[query]`, `#[update]`)
- **types.rs**: Contains all data structures including `Todo`, input/output types, and implements `Storable` trait for stable storage
- **storage.rs**: Manages stable storage using `StableBTreeMap` for persistent data across canister upgrades
- **service.rs**: Business logic layer with validation functions and CRUD operations
- **todo_ic_backend.did**: Candid interface definition for external API interaction

#### Frontend (`src/todo_ic_frontend/`)

- **lib.rs**: Main library entry point with WASM hydration setup
- **app.rs**: Root Leptos component with overall app structure
- **components/**: Modular UI components
  - **todo_form.rs**: Add new todo form with validation
  - **todo_list.rs**: Display and manage existing todos
  - **network_selector.rs**: Network switcher (Local/Testnet/Mainnet)
- **ic_client.rs**: Internet Computer client for API communication
- **types.rs**: Shared type definitions matching backend types
- **style/main.css**: Colorful responsive CSS styling

## API Endpoints

| Method | Type | Description |
|--------|------|-------------|
| `add_todo(CreateTodoInput)` | Update | Create a new todo item |
| `get_todo(TodoId)` | Query | Retrieve a specific todo by ID |
| `get_all_todos(PaginationInput)` | Query | Get paginated list of todos |
| `update_todo_text(TodoId, text)` | Update | Update todo text content |
| `update_todo_completed(TodoId, bool)` | Update | Mark todo as complete/incomplete |
| `delete_todo(TodoId)` | Update | Delete a todo item |
| `get_todo_count()` | Query | Get total number of todos |

## Makefile Commands

The project includes a comprehensive Makefile with the following commands:

### Development Commands

```bash
make help               # Show all available commands

# Backend
make build-backend     # Build the backend canister
make test              # Run all Rust tests
make lint              # Run Clippy linter for code quality
make fmt               # Format code using rustfmt

# Frontend
make build-frontend    # Build the Leptos frontend
make serve-frontend    # Start frontend development server
make install-frontend-deps # Install trunk and wasm target

# Combined
make build             # Build both backend and frontend
```

### Local Development

```bash
make start         # Start DFX replica in background
make deploy        # Build and deploy canister locally
make test-api      # Test API endpoints with sample data
make stop          # Stop the local DFX replica
make clean         # Clean build artifacts and local state
```


### Mainnet Deployment

```bash
make deploy-mainnet         # Deploy to Internet Computer mainnet
make deploy-mainnet-cycles  # Deploy to mainnet with cycles (interactive)
```

### Example Workflows

#### Local Development Workflow

```bash
# Start development
make start
make deploy

# Test the API
make test-api

# Run tests
make test

# Clean up
make stop
make clean
```

#### Testnet Deployment Workflow

```bash
# Deploy to testnet (requires cycles from faucet)
make setup-testnet

# Test your deployment
make test-api-testnet
```

#### Mainnet Deployment Workflow

```bash
# Deploy to mainnet (requires real ICP/cycles)
make deploy-mainnet-cycles
```

## Setup and Usage

1. **Prerequisites**:
   ```bash
   # Install DFX
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

2. **Local Development**:
   ```bash
   # Start backend
   make start
   make deploy

   # Start frontend (in another terminal)
   make serve-frontend
   ```

3. **Test the API**:
   ```bash
   # Add a todo
   dfx canister call todo_ic_backend add_todo '(record { text = "Learn IC development" })'

   # Get all todos
   dfx canister call todo_ic_backend get_all_todos '(record { offset = 0; limit = 10 })'

   # Update completion status
   dfx canister call todo_ic_backend update_todo_completed '(1, true)'
   ```

## Testnet Deployment

The project is configured to deploy to IC Playground (testnet) at `https://icp0.io`.

### Prerequisites for Testnet Deployment

1. **Get Free Cycles**: Visit [DFINITY Faucet](https://faucet.dfinity.org/) to get free cycles for testing

### Deployment Process

```bash
# Complete testnet deployment
make setup-testnet
```

### Testing Your Deployed Canister

```bash
# Test API endpoints (automatically detects your canister ID)
make test-api-testnet
```

### Accessing Your Testnet Canister

After deployment with `make setup-testnet`, your canister URLs will be displayed in the output:
- **Direct URL**: `https://<canister-id>.icp0.io`
- **Candid UI**: `https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=<canister-id>`

## Mainnet Deployment

**Mainnet Principal ID**: `vyggl-hpuoy-22jqe-6povn-6glf6-z2rle-wgphx-nyv6a-xbomq-e6knd-iae`

To deploy to mainnet:
1. Fund the principal with ICP tokens
2. Convert ICP to cycles: `dfx cycles convert <amount>`
3. Deploy: `make deploy-mainnet`

## Features

### Backend Features
- **Persistent Storage**: Uses `StableBTreeMap` for upgrade-safe data persistence
- **Pagination**: Efficient handling of large todo lists
- **Input Validation**: Comprehensive validation with proper error handling
- **Timestamps**: Automatic creation and update timestamps
- **Type Safety**: Full Rust type system with Candid integration
- **Comprehensive Testing**: Full test coverage for all CRUD operations

### Frontend Features
- **🚀 Modern UI**: Built with Leptos (Rust WASM framework)
- **🌐 Network Switching**: Toggle between Local/Testnet/Mainnet
- **🎨 Colorful Design**: Responsive design with gradient backgrounds and animations
- **⚡ Real-time Updates**: Reactive UI with immediate feedback
- **📱 Mobile Friendly**: Responsive design works on all devices
- **🔄 Loading States**: Visual feedback for all async operations
- **❌ Error Handling**: User-friendly error messages and recovery

## Technical Details

### Storage Implementation
- Uses IC stable structures for persistent storage across canister upgrades
- `TodoStorage`: Maps `TodoId` to `Todo` objects
- `IdStorage`: Manages auto-incrementing ID counter
- Memory management with separate virtual memory regions

### Data Types
- `TodoId`: 64-bit unsigned integer identifier
- `Todo`: Main todo structure with id, text, completion status, and timestamps
- Result types for proper error handling (`TodoResult`, `TodosResult`, `DeleteResult`)

### Validation Rules
- Todo text: 1-1000 characters, non-empty after trimming
- Pagination: limit 1-100, offset >= 0
- Proper error messages for all validation failures