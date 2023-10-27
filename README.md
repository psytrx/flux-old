# Flux

Physically based ray tracing adventures in Rust.

## Getting Started

## Usage

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

## Example Scenes

Available scenes:

- CornellBox
- DefocusBlur
- Dragon
- ManySpheres
- MaterialDemo
- Suzanne

```bash
flux --scene cornellbox
```

Also, see the [example renders](./examples/) in this repository.
