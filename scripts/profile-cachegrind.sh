#!/bin/bash
set -e

cargo build --profile profiling
RUST_LOG=debug valgrind --tool=cachegrind ./target/profiling/flux --dev --scene cornellbox
