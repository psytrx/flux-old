#!/bin/bash

cargo build --profile profiling &&
	RUST_LOG=debug samply record target/profiling/flux
