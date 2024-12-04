default:
    @just --list

# Build project
build:
    cargo build

# Check code with clippy
lint:
    cargo clippy -- -D warnings

# Deploy app to shuttle
deploy:
    cargo shuttle deploy

# Run project locally
run:
    cargo shuttle run

