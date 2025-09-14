# Todo IC

A simple todo list canister for the Internet Computer built with Rust.

## Quick Start

### Local Development
Get the local development environment ready with one command:

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
1. Checks cycles balance
2. Create testnet canister
3. Build the backend
4. Deploy to testnet

Canister will be ready for testing with API endpoints available!

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

## Setup and Usage

1. **Prerequisites**:
   ```bash
   # Install DFX
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   
   # Install Trunk (for frontend builds)
   cargo install trunk
   ```

2. **Local Development**:
   ```bash
   # Complete local setup (stop, start, create, build, deploy)
   make setup-local
   ```

3. **Building Frontend + Backend**:
   ```bash
   # Build both backend and frontend (backend builds first, then frontend)
   make build
   
   # Or build individually:
   make build-backend    # Build only backend canister
   make build-frontend   # Build only frontend (automatically builds backend first)
   
   # Serve frontend development server with hot reload
   make serve-frontend
   ```

4. **Test API Commands**:
   ```bash
   # Add a todo
   dfx canister call todo_ic_backend add_todo '(record { text = "Learn IC development" })'

   # Get a specific todo by ID
   dfx canister call todo_ic_backend get_todo '(1)'

   # Get all todos with pagination
   dfx canister call todo_ic_backend get_all_todos '(record { offset = 0; limit = 10 })'

   # Update todo text
   dfx canister call todo_ic_backend update_todo_text '(1, "Updated todo text")'

   # Mark todo as completed
   dfx canister call todo_ic_backend update_todo_completed '(1, true)'

   # Get total todo count
   dfx canister call todo_ic_backend get_todo_count '()'

   # Delete a todo
   dfx canister call todo_ic_backend delete_todo '(1)'

   # Test all endpoints with make command
   make test-api
   ```

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