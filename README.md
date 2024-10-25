# Flux

Physically based ray tracing adventures in Rust.

## Getting Started

### Usage

```bash
$ flux --help

Usage: flux [OPTIONS]

Options:
  -s, --scene <SCENE>
          The example scene to render [default: cornellbox]
      --sweeps <SWEEPS>
          Number of full CPU sweeps to run [default: 16]
      --spp <SPP>
          Samples/pixel/pass [default: 16]
      --min-depth <MIN_DEPTH>
          Minimum tracing path depth [default: 8]
      --max-depth <MAX_DEPTH>
          Maximum tracing path depth [default: 32]
      --rr-stop-prob <RR_STOP_PROB>
          Russian roulette path termination probability [default: 0.1]
      --aux-spp <AUX_SPP>
          Samples/pixel/pass for auxiliary channels [default: 4]
  -o, --out-dir <OUT_DIR>
          Output directory for rendered images [default: ./output]
  -u, --update-interval <UPDATE_INTERVAL>
          Update interval for intermediate render results [default: 1]
      --dev
          Switch for running quick debug renders. Overrides most of the other settings
  -h, --help
          Print help
  -V, --version
          Print version
```

### Utility Scripts

```bash
# set execute permissions
chmod u+x ./scripts/*

# build & profile the profiling build (requires samply & valgrind to be installed)
./scripts/profile-samply.sh
./scripts/profile-cachegrind.sh

# build & run release build
./scripts/run.sh
# you can also pass arguments to the run script
./scripts/run.sh --scene dragon --spp 16 --sweeps 8

# build & run debug/development build
# presets with low-quality, but fast renderings
./scripts/run-dev.sh --scene suzanne
```

### Logging

Flux uses [env_logger](https://docs.rs/env_logger/) for logging.
To enable logging, set the corresponding environment variables to control verbosity:

```bash
RUST_LOG=debug flux [OPTIONS]
# available options: trace, debug, info, warn, error
```

## Example Scenes

Currently, flux does not provide a way to load custom scenes.
However, this is on the roadamap for the near future.

Until then, you can load one of the example scenes:

- CornellBox
- DefocusBlur
- Dragon
- ManySpheres
- MaterialDemo
- SimpleCornellBox
- Suzanne

```bash
flux --scene cornellbox
```

Also, see the [example renders](./examples/) in this repository.
