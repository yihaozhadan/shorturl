# ShortURL Backend Service

A URL shortening backend service implemented in Rust, using Actix Web for HTTP handling and SurrealDB as the data store.

This repository is currently driven by specifications under `specs/001-build-backend-service/`. The implementation in `src/` and `tests/` should follow those specs and tasks.

## Project Status

Implementation is guided by:

- `specs/001-build-backend-service/spec.md`
- `specs/001-build-backend-service/plan.md`
- `specs/001-build-backend-service/tasks.md`

If code has not yet been created, start with the tasks file and follow the phases in order.

## Tech Stack (planned)

- Rust
- Actix Web (HTTP server and routing)
- SurrealDB (database)
- Serde (serialization)

## Getting Started

1. Install Rust (via [rustup](https://rustup.rs/)).
2. Clone this repository:
   ```bash
   git clone https://github.com/yihaozhadan/shorturl.git
   cd shorturl
   ```
3. Initialize the Rust project structure if it does not exist yet (see `specs/001-build-backend-service/tasks.md`, Phase 1).

Once the Cargo project is set up as described in the specs, you should be able to:

```bash
cargo build
cargo run
```

## Running Tests

After tests are implemented under `tests/` as described in `tasks.md`:

```bash
cargo test
```

## Directory Structure (planned)

- `src/`
  - `main.rs` – Actix application entrypoint
  - `routes/` – HTTP handlers and routing
  - `services/` – business logic (URL shortening, validation, etc.)
  - `db/` – SurrealDB client and repositories
  - `config/` – configuration and logging
- `tests/` – unit and integration tests
- `specs/001-build-backend-service/` – specs, plans, and tasks driving implementation

## Implementation Guide

Use `specs/001-build-backend-service/tasks.md` as the canonical checklist. Complete phases in order:

1. Setup (Phase 1)
2. Foundational (Phase 2)
3. User Story 1 (Phase 3)
4. User Story 2 (Phase 4)
5. User Story 3 (Phase 5)
6. Polish & Cross-Cutting Concerns (Phase 6)

Commit after each completed task or logical group and validate at the checkpoints defined in `tasks.md`.
