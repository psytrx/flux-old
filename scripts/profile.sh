#!/bin/bash

cargo build --profile profiling &&
	RUST_LOG=debug samply record target/profiling/flux

# cargo build --profile profiling &&
# 	RUST_LOG=debug valgrind --tool=cachegrind ./target/release/flux --dev

# cargo build --profile profiling &&
# 	RUST_LOG=debug valgrind --tool=dhat ./target/release/flux --dev
