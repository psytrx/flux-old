#!/bin/bash
set -e

cargo build --profile release
RUST_LOG=trace target/release/flux "$@"
