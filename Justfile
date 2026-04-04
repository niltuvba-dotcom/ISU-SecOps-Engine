# Justfile - Aetheris Engine Professional Command Runner

_default:
    @just --list

# Format all code including TOML
fmt:
    cargo fmt --all
    taplo format

# Run clippy lints with strict security rules
lint:
    cargo clippy --all-targets -- -D warnings

# Run all unit tests
test:
    cargo test --workspace

# Check for security vulnerabilities in dependencies
audit:
    cargo deny check

# Build the project in release mode for maximum performance
build:
    cargo build --release

# Quick scan of localhost (Terminal focus)
scan target='127.0.0.1' ports='1-1000':
    cargo run -- pentest fingerprint {{target}} --ports {{ports}}

# Run the full local CI gate manually
ci: fmt lint audit test build
    @echo "✅ Aetheris Engine: All local CI checks passed! Deployment ready."

# Start the Web Dashboard companion
web port='8080':
    cargo run -- web --port {{port}}
