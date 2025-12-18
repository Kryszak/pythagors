[default]
_list_available:
    @just --list --unsorted

# Build project
[group('CI')]
build:
    cargo build

# Check code with clippy
[group('CI')]
lint:
    cargo clippy -- -D warnings

# Run project locally
[group('local')]
run:
    -cargo run

