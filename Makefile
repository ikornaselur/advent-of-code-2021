day%:
	@cargo run --release --bin day$*

test_day%:
	@cargo test --bin day$*
