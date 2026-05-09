.PHONY: all build build-release test clean run fmt lint update install

# Default target
all: build

# Build the main flexo binary in debug mode
build:
	cd flexo && cargo build

# Build the main flexo binary in release mode
build-release:
	cd flexo && cargo build --release

# Run all tests (unit and integration)
test:
	cd flexo && cargo test

# Run the flexo server
run:
	cd flexo && cargo run

# Clean build artifacts for all components
clean:
	cd flexo && cargo clean
	cd test/integration-test-client && cargo clean
	cd test/tcp-proxy-delay && cargo clean

# Format code across the project
fmt:
	cd flexo && cargo fmt
	cd test/integration-test-client && cargo fmt
	cd test/tcp-proxy-delay && cargo fmt

# Run clippy for linting
lint:
	cd flexo && cargo clippy -- -D warnings
	cd test/integration-test-client && cargo clippy -- -D warnings
	cd test/tcp-proxy-delay && cargo clippy -- -D warnings

# Update rust toolchain and dependencies
update:
	rustup update
	cd flexo && cargo update
	cd test/integration-test-client && cargo update
	cd test/tcp-proxy-delay && cargo update

# Build integration test utilities
build-utils:
	cd test/integration-test-client && cargo build
	cd test/tcp-proxy-delay && cargo build

# Install the binary and service file (requires sudo)
install: build-release
	install -Dm755 flexo/target/release/flexo /usr/bin/flexo
	install -Dm644 flexo/etc/flexo.service /usr/lib/systemd/system/flexo.service
	mkdir -p /etc/flexo
	[ -f /etc/flexo/flexo.toml ] || install -m644 flexo/conf/flexo.toml /etc/flexo/flexo.toml
	mkdir -p /var/cache/flexo/pkg
	mkdir -p /var/cache/flexo/state
