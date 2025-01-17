#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

DIR_PS=${DIR%/*/*/*}/ProtoScript

$DIR_PS/build.sh

cd proto

$DIR_PS/packages/protoscript/dist/cli/index.js
