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


# Create project on shuttle
[group('Shuttle')]
create-project:
    cargo shuttle project start

# Deploy app to shuttle
[group('Shuttle')]
deploy:
    cargo shuttle deploy --allow-dirty

# Stop shuttle deployment
[group('Shuttle')]
stop:
    cargo shuttle stop

# Run project locally
[group('local')]
run:
    cargo shuttle run

