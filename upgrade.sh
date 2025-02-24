#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

ncu -u
bun i
cd ./coffee/rust2proto
ncu -u
bun i

gcip() {
  git add -u && git commit -m "upgrade" && git pull && git push || true
}

cd $DIR

./rust/sh/upgrade.sh
gcip

cd ../mod
./sh/upgrade.sh
gcip

cd ../srv
./sh/upgrade.sh
gcip
