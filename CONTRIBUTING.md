# Contributing

## Requirements

Install `rust` 🦀 : <https://doc.rust-lang.org/cargo/getting-started/installation.html>

## Install

```sh
pnpm i
```

## Testing

```sh
cargo test
```

## Compiling rust project

```sh
pnpm compile
```

## Linking local build

```sh
pnpm link --global
```

## How does it work?

`pnpm` runs the cargo build command and moves the executables in the `dist` folder of the subpackage `npm`.

`cargo` build the crates defined in `Cargo.toml`.
