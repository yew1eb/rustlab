
build:
	cargo fmt --
	cargo build --all-features --workspace  --exclude third-party
	cargo test --all-features --workspace  --exclude third-party
	cargo clippy --all-features