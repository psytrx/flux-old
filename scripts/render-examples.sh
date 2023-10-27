#!/bin/bash
set -e

scenes=("cornellbox" "defocusblur" "dragon" "manyspheres" "materialdemo" "suzanne")

for scene in "${scenes[@]}"; do
	echo "Rendering $scene"

	cargo build --profile release
	RUST_LOG=trace target/release/flux --sweeps 4 --spp 1 --aux-spp 4 --scene "$scene"
	cp ./output/output.png ./examples/img/"$scene".png

done
