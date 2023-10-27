# Flux

Physically based ray tracing adventures in Rust.

## Getting Started

## Usage

```bash
cargo build --profile release
./target/release/flux --help
# or:
# cargo run --release -- --help

  Usage: flux [OPTIONS] --scene <SCENE>

  Options:
    -d, --dev            Runs quick/noisy renders for iterating quickly
    -s, --scene <SCENE>  The example scene to render
    -h, --help           Print help
    -V, --version        Print version
```

## Example Scenes

Available scenes:

- MaterialDemo
- DefocusBlur
- ManySpheres
- CornellBox
- Suzanne
- Dragon

```bash
flux --scene cornellbox
```

Also, see the [example renders](./examples/) in this repository.
