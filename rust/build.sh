#!/usr/bin/env bash

DIR=$(cd "$(dirname "$0")"; pwd)
cd $DIR
set -ex

npm run build-release
./test/index.coffee
