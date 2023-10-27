#!/bin/bash
set -e

cargo build --profile release
RUST_LOG=trace target/release/flux --sweeps 4 --spp 1 --aux-spp 1 "$@"
