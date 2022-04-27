.EXPORT_ALL_VARIABLES:
RUST_LOG=sqlx=warn,info

fmt:
	cargo fmt --all -- --check

build:
	cargo build

db:
	cargo run --bin migrate database reset --source src/db/migrations

test:
	cargo test

run:
	cargo run --bin realworld_application

preflight: fmt clippy test

clippy:
	cargo clippy --all -- -D warnings

