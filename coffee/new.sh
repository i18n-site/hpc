#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR

if [ -d "$1" ]; then
  echo "$1 EXIST"
  exit 1
fi

set -ex

cp -R _tmpl $1

cd $1

rpl _tmpl $1

git add .
