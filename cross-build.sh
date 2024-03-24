#!/bin/bash
# targets=("x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu" "aarch64-apple-darwin")
targets=("aarch64-apple-darwin" "x86_64-apple-darwin" "x86_64-unknown-linux-gnu")
for target in "${targets[@]}"
do
  cross build --target $target --workspace --bins -r
  cp target/$target/release/* npm/$target/dist
done