#!/usr/bin/env bash

docker run --rm -it $(docker build -q .) $@
