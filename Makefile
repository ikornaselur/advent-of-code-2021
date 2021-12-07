day%:
	@cargo run --release --bin day$*

test_day%:
	@cargo test --bin day$*

lint: 
	@cargo clippy -- -D warnings

fmt:
	@cargo fmt -- --check
