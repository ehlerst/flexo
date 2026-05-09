# Flexo Project Context

Flexo is a caching proxy for Arch Linux's package manager, `pacman`. It automatically selects the fastest mirrors through latency testing and enables efficient parallel downloads by caching packages and sharing bandwidth among concurrent clients.

## Project Overview

- **Main Technologies:** Rust (2018 edition), `curl` (via `curl-rust`), `crossbeam` (concurrency), `httparse` (HTTP parsing), `serde` (serialization), `tokio` (not used directly in core, but uses standard threads and channels).
- **Core Architecture:**
  - **Flexo Library (`flexo/src/lib.rs`):** A generic framework defining traits for `Provider` (mirrors), `Job` (downloads), `Order` (resource requests), and `Channel` (connections). It manages job scheduling, retries, and provider scoring.
  - **Flexo Server (`flexo/src/main.rs`):** The concrete application that implements the server-side logic, including a TCP listener, HTTP request handling, and cache management.
  - **Mirror Implementation (`flexo/src/mirror_flexo.rs`):** Implements the generic traits specifically for Arch Linux mirrors and `pacman` requests.
  - **Configuration (`flexo/src/mirror_config.rs`):** Loads settings from `/etc/flexo/flexo.toml` or environment variables (prefixed with `FLEXO_`).

## Key Directories and Files

- `flexo/src/`: Core source code.
  - `lib.rs`: Generic job/provider management traits.
  - `main.rs`: Entry point and HTTP server.
  - `mirror_flexo.rs`: Arch Linux specific implementations.
  - `mirror_config.rs`: Configuration management.
  - `mirror_fetch.rs`: Mirror list retrieval and latency testing.
- `flexo/conf/flexo.toml`: Default configuration file.
- `flexo/terminology.md`: Detailed explanation of the project's domain-specific terms.
- `test/`: Integration and end-to-end tests, including Docker-based test setups.

## Building and Running

### Prerequisites
- Rust and Cargo.
- `curl`, `paccache` (or `scruffy`) for cache cleaning.

### Commands
- **Build:** `cargo build` (run from the `flexo/` directory).
- **Run:** `cargo run` (run from the `flexo/` directory).
- **Unit & Integration Tests:** `cargo test` (run from the `flexo/` directory).
- **Docker E2E Tests:** `cd test/docker-test-local && ./docker-compose` (Requires 32GB RAM and Docker BuildKit).

## Development Conventions

- **Rust Edition:** 2018.
- **Concurrency:** Uses a thread-per-connection model for the server and `crossbeam` channels for communication between the scheduling logic and worker threads.
- **Efficiency:** Utilizes `sendfile64` for zero-copy file transfers from cache to clients.
- **Error Handling:** Uses custom `Result` types (`JobResult`, `ClientError`) and extensive logging (`log` crate with `env_logger`).
- **Cache Integrity:** Uses `.cfs` (Complete File Size) files to track the expected size of partially downloaded packages.
- **Testing:** Before submitting PRs, ensure `cargo test` passes in the `flexo/` directory.

## Configuration Highlights

- `cache_directory`: Where packages are stored (default: `/var/cache/flexo/pkg`).
- `mirror_selection_method`: "auto" (latency-based) or "predefined".
- `mirrors_auto`: Configures how mirrors are discovered and tested.
- `num_versions_retain`: Number of versions per package to keep in cache (default: 3).
- `listen_ip_address`: Default is `127.0.0.1` (change to `0.0.0.0` for LAN access).
- `port`: Default is `7878`.
