#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if ! command -v pbc &>/dev/null; then
  cargo install pbc
fi
pbc
rm -rf pb
mv rust/proto__/src/proto__.rs src/proto.rs
rm -rf rust
cargo fmt

cd $DIR/proto
bun x protoscript
