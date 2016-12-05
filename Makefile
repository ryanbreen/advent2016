.PHONY: all build run test

all: $(build)

build:
	@cargo build --release

export RUST_MIN_STACK=33554432
test:
	@cargo test --release
