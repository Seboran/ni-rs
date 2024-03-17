#!/bin/bash
targets=("x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu" "aarch64-apple-darwin")
for target in "${targets[@]}"
do
  cross build --target $target --workspace --bins -r
done