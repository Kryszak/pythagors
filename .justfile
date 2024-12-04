default:
    @just --list

# Build project
[group('CI')]
build:
    cargo build

# Check code with clippy
[group('CI')]
lint:
    cargo clippy -- -D warnings

# Deploy app to shuttle
[group('Shuttle')]
deploy:
    cargo shuttle deploy

# Run project locally
[group('local')]
run:
    cargo shuttle run

