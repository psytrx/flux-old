#!/bin/bash
set -e

cargo build --profile profiling
RUST_LOG=debug samply record target/profiling/flux --scene cornellbox
