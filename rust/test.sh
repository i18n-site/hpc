#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

if [ ${#1} -eq 0 ]; then
  if [ -f ".dev" ]; then
    arg=$(cat .dev)
  else
    echo "❯ $0 项目名"
    exit 1
  fi
else
  echo $@ >.dev
  arg=$@
fi

set -ex

cd $arg
cargo nextest run --all-features --nocapture
