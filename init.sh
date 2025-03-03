#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if [ -d ".git" ]; then
  cd ..
else
  mkdir -p i18n
  cd i18n
fi

bun_i() {
  if ! command -v $(basename $1) &>/dev/null; then
    bun i -g $1
  fi
}

bun_i @antfu/ni

clone() {
  for i in "$@"; do
    if [ -d "$i" ]; then
      cd $i
      git add . && git commit -m "." || true
      git pull || true
      git push
    else
      git clone --depth 1 --no-single-branch git@github.com:i18n-site/$i.git
      cd $i
      git fetch origin dev --depth 1
      git checkout -b dev origin/dev
      [ -f .mise.toml ] && mise trust
      fd package.json -x bash -c 'cd $(dirname {}) && ni'
    fi
    cd ..
  done
}

clone web srv mod hpc
