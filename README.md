# ShortURL Service

A full-stack URL shortening service with a Rust backend, SvelteKit frontend, and SurrealDB database. Deployable via Docker Compose for easy production setup.

This repository is currently driven by specifications under `specs/001-build-backend-service/`. The implementation in `src/` and `tests/` should follow those specs and tasks.

## Project Status

Implementation is guided by:

- `specs/001-build-backend-service/spec.md`
- `specs/001-build-backend-service/plan.md`
- `specs/001-build-backend-service/tasks.md`

If code has not yet been created, start with the tasks file and follow the phases in order.

## Tech Stack

### Backend
- Rust
- Actix Web (HTTP server and routing)
- SurrealDB (database)
- Serde (serialization)

### Frontend
- SvelteKit
- TailwindCSS
- TypeScript

### Infrastructure
- Docker & Docker Compose
- SurrealDB (file-based storage)

## Quick Start with Docker (Recommended)

The easiest way to run the entire stack is using Docker Compose:

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yihaozhadan/shorturl.git
   cd shorturl
   ```

2. **Create environment file:**
   ```bash
   cp .env.example .env
   ```
   
   Edit `.env` if you want to customize credentials or ports.

3. **Start all services:**
   ```bash
   docker-compose up -d
   ```

4. **Access the application:**
   - Frontend: http://localhost:3000
   - Backend API: http://localhost:8080
   - SurrealDB: http://localhost:8000

5. **View logs:**
   ```bash
   docker-compose logs -f
   ```

6. **Stop all services:**
   ```bash
   docker-compose down
   ```

### Data Persistence

SurrealDB data is persisted to `./data/database.db` on your host machine. To reset the database, stop the containers and delete the `data` directory:

```bash
docker-compose down
rm -rf data
```

## Development Setup (Local)

### Prerequisites

- Rust (via [rustup](https://rustup.rs/))
- Node.js 22+ (via [nvm](https://github.com/nvm-sh/nvm))
- SurrealDB CLI (via [SurrealDB installation](https://surrealdb.com/install))

### Backend Development

1. **Start SurrealDB:**
   ```bash
   surreal start --user root --pass root file://mydb
   ```

2. **Start the backend server:**
   ```bash
   cargo run
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

### Frontend Development

1. **Install dependencies:**
   ```bash
   cd frontend
   npm install
   ```

2. **Set up environment:**
   ```bash
   cp .env.example .env
   ```

3. **Start development server:**
   ```bash
   npm run dev
   ```

4. **Run tests:**
   ```bash
   npm test
   ```

## Production Deployment

### Environment Variables

The application uses the following environment variables (see `.env.example`):

#### SurrealDB Configuration
- `SURREALDB_USER` - Database username (default: `root`)
- `SURREALDB_PASS` - Database password (default: `root`)
- `SURREALDB_NS` - Database namespace (default: `shorturl`)
- `SURREALDB_DB` - Database name (default: `shorturl`)

#### Backend Configuration
- `SHORTURL_SERVER_ADDR` - Backend server address (default: `127.0.0.1:8080`)
- `SURREALDB_URL` - SurrealDB connection URL (default: `127.0.0.1:8000`)
- `RUST_LOG` - Logging level (default: `info`)

#### Frontend Configuration
- `BACKEND_URL` - Internal backend URL (server-side only)
- `PUBLIC_BACKEND_URL` - Public-facing backend URL for short links
- `ORIGIN` - SvelteKit origin URL for CORS
- `PORT` - Frontend server port (default: `3000`)
- `HOST` - Frontend server host (default: `0.0.0.0`)

### Building for Production

#### Backend
```bash
cargo build --release
./target/release/shorturl
```

#### Frontend
```bash
cd frontend
npm ci --production
npm run build
node build
```

#### Docker Compose (Recommended)
```bash
# Build and start all services
docker-compose up -d --build

# View service status
docker-compose ps

# Scale services (example: 3 backend instances)
docker-compose up -d --scale backend=3
```

### Health Checks

- Backend health: `GET http://localhost:8080/`
- SurrealDB health: `GET http://localhost:8000/health`

## Directory Structure

### Backend
- `src/`
  - `main.rs` – Actix application entrypoint
  - `routes/` – HTTP handlers and routing
  - `services/` – business logic (URL shortening, validation, etc.)
  - `db/` – SurrealDB client and repositories
  - `config/` – configuration and logging
- `tests/` – unit and integration tests
- `db/schema.surql` – Database schema
- `Dockerfile` – Backend production container

### Frontend
- `frontend/`
  - `src/` – SvelteKit application source
  - `static/` – Static assets
  - `Dockerfile` – Frontend production container

### Infrastructure
- `docker-compose.yml` – Full stack orchestration
- `.env.example` – Environment variable template
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

## Troubleshooting

### Docker Issues

**Container fails to start:**
```bash
# Check logs
docker-compose logs backend
docker-compose logs frontend
docker-compose logs surrealdb

# Rebuild containers
docker-compose down
docker-compose up -d --build
```

**Database connection errors:**
- Ensure SurrealDB container is healthy: `docker-compose ps`
- Check credentials in `.env` match the database configuration
- Verify network connectivity: `docker-compose exec backend ping surrealdb`

**Port conflicts:**
- Check if ports 3000, 8000, or 8080 are already in use
- Modify ports in `docker-compose.yml` if needed

### Development Issues

**Backend compilation errors:**
```bash
cargo clean
cargo build
```

**Frontend build errors:**
```bash
cd frontend
rm -rf node_modules .svelte-kit
npm install
npm run build
```

## Contributing

1. Follow the specifications in `specs/001-build-backend-service/`
2. Write tests for new features
3. Ensure all tests pass: `cargo test` and `npm test`
4. Follow the existing code style
5. Update documentation as needed

## License

MIT License
