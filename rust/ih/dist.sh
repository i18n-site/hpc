#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

./gen.sh
../cargo.dist.sh $(basename $DIR)
