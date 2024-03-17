#!/bin/bash
targets=("x86_64-unknown-linux-gnu" "x86_64-pc-windows-gnu")
for target in "${targets[@]}"
do
  cross build --target $target --workspace --bins -r
done
#"aarch64-apple-darwin"