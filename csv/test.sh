#!/usr/bin/env bash

_DIR=$(dirname $(realpath "$0"))
cd $_DIR

npx coffee -m --compile --output lib src/

nr prepare

tape test/**/*.coffee|npx colortape
