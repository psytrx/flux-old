#!/bin/bash

cargo build --profile release &&
	RUST_LOG=trace target/profiling/flux
