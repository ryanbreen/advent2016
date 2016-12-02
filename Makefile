.PHONY: all build run test

all: $(build)

build:
	@cargo build

run: $(build)
	cargo run -- -d $(day) -p $(part)

export RUST_MIN_STACK=33554432
test:
	@cargo test
