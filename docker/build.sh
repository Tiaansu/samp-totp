#!/bin/bash

docker build \
    -t samp-totp/build:ubuntu-18.04 ./ \
|| exit 1

docker run \
    --rm \
    -t \
    -w /code \
    -v $PWD/..:/code \
    samp-totp/build:ubuntu-18.04