# Contributing

## Requirements

1. Install `bun` : <https://bun.sh/docs/installation>
2. Install `rust` 🦀 : <https://doc.rust-lang.org/cargo/getting-started/installation.html>

## Install

```sh
bun i
```

## Testing

```sh
cargo test
```

## Compiling rust project

```sh
bun compile
```

## Linking local build

```sh
bun link:local
```

## How does it work?

`Bun` runs the cargo build command and moves the executables in the `dist` folder of the subpackage `npm`.

`cargo` build the crates defined in `Cargo.toml`.
