# FIXME: Improvements and Best Practices

This document tracks identified issues, code smells, and areas for improvement in the Flexo codebase.

## Resilience and Error Handling
- [x] **Eliminate Panics**: Replace `unwrap()`, `expect()`, and `panic!()` with proper `Result` propagation. A server should not crash due to a missing environment variable or a temporary I/O error.
  - [x] `mirror_config.rs`: `mirror_config_from_env` panics if any `FLEXO_*` variable is missing.
  - [x] `main.rs`: `initialize_job_context` panics on various I/O or configuration failures.
  - [x] `fs_utils.rs`: `create_dir_unless_exists` panics on failure.
- [x] **Unified Error Types**: Use a crate like `thiserror` to define structured error types across the project instead of manual boilerplate.

## Performance and Architecture
- [ ] **Async I/O**: Consider moving from a thread-per-connection model to an async runtime like `tokio`. This would allow Flexo to handle significantly more concurrent clients with lower memory overhead.
- [x] **Thread Pool**: If staying with synchronous threads, use a thread pool (e.g., `rayon` or `threadpool`) instead of spawning a new thread for every client connection to prevent resource exhaustion attacks.
- [ ] **Mutex Contention**: Audit `Arc<Mutex<JobContext>>` and other global locks. High contention on these locks could bottleneck performance during parallel downloads.

## Security
- [x] **`unsafe` Audit**: The `sendfile64` implementation in `main.rs` needs a rigorous audit to ensure it correctly handles all edge cases (e.g., 32-bit overflows, unexpected file truncations).
- [x] **Path Traversal**: Review `permitted_path` and `valid_path` in `main.rs` to ensure they are robust against advanced path traversal techniques. (Audited: components() check is robust against .. and .).

## Portability and Configuration
- [x] **Hardcoded Paths**: Remove hardcoded paths like `/etc/flexo/flexo.toml` and `/var/cache/flexo`. Use defaults that can be overridden via command-line arguments or environment variables.
- [ ] **SI Unit Parsing**: Replace custom bandwidth parsing in `mirror_config.rs` with a more robust library or a more comprehensive implementation.
- [x] **Env Var Parsing**: Refactor `parse_env_toml` to avoid the "fake TOML document" hack. Use a dedicated environment variable parsing library or simpler direct parsing.

## Code Quality and Maintainability
- [x] **Internal Mirror Logic**: In `mirror_fetch.rs`, the hardcoded path `core/os/x86_64/core.db` for latency testing should be configurable.
- [x] **Cloudflare Detection**: The Cloudflare server header check in `mirror_fetch.rs` is brittle and should be made case-insensitive and more robust.
- [x] **Dependency Audit**: Many dependencies in `Cargo.toml` are outdated. Regularly run `cargo update` and consider upgrading major versions where beneficial.
- [ ] **Unified Metrics**: Unify the internal `ProviderMetrics` used for mirror selection with the newly added Prometheus metrics to avoid redundant data tracking.

## Tests
- [ ] **Deterministic Tests**: Address the note in `test/docker-test-local/README.md` about non-deterministic end-to-end tests.
- [ ] **Mocking**: Increase the use of mocked network responses in unit tests to reduce reliance on external mirrors during testing.
