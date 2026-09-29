# rust-pipeline-orchestrator

A pipeline orchestration service written in Rust, built on [Axum](https://github.com/tokio-rs/axum), [Tokio](https://tokio.rs) and [SQLx](https://github.com/launchbadge/sqlx) with PostgreSQL.

> **Status:** early scaffold. The HTTP server, configuration, database pool and domain model are in place. API routes, migrations and the background worker are not implemented yet.

## Tech stack

| Concern       | Crate                              |
| ------------- | ---------------------------------- |
| HTTP server   | `axum` 0.7, `tower-http` (tracing) |
| Async runtime | `tokio`                            |
| Database      | `sqlx` 0.8 (Postgres)              |
| Serialization | `serde`, `serde_json`              |
| Errors        | `thiserror`                        |
| Logging       | `tracing`, `tracing-subscriber`    |
| Config        | `dotenvy`                          |

## Project layout

```
src/
├── main.rs            # Entry point: loads config, connects to the DB, starts the server
├── config.rs          # Environment-based configuration
├── state.rs           # Shared application state (DB pool)
├── error.rs           # AppError and its HTTP response mapping
├── api/               # Axum router
├── database/          # Postgres connection pool
├── domain/
│   └── pipeline.rs    # Pipeline model
└── worker/            # Background worker (stub)
```

## Getting started

### Prerequisites

- Rust (stable, 2021 edition)
- Docker with Docker Compose

### 1. Configure the environment

```sh
cp .env.example .env
```

Set `POSTGRES_PASSWORD` in `.env` and use the same password in `DATABASE_URL`.

| Variable            | Required | Default     | Description                              |
| ------------------- | -------- | ----------- | ---------------------------------------- |
| `DATABASE_URL`      | yes      | —           | Postgres connection string for the app   |
| `SERVER_HOST`       | no       | `localhost` | Address the HTTP server binds to         |
| `SERVER_PORT`       | no       | `3000`      | Port the HTTP server listens on          |
| `POSTGRES_USER`     | no       | `pipeline`  | Postgres user created by docker-compose  |
| `POSTGRES_PASSWORD` | yes      | —           | Postgres password used by docker-compose |
| `POSTGRES_DB`       | no       | `pipeline`  | Database created by docker-compose       |

### 2. Start Postgres

```sh
docker compose up -d
```

Postgres is exposed on `localhost:4432`, and its data is kept in the `postgres-data` volume.

### 3. Run the server

```sh
cargo run
```

The server logs its address on startup, for example `Server running on 127.0.0.1:3000`.

## Development

```sh
cargo fmt       # format
cargo clippy    # lint
cargo test      # run tests
```

## Roadmap

- [ ] Database migrations for pipelines
- [ ] CRUD API for pipelines
- [ ] Background worker for pipeline execution
- [ ] Health check endpoint
