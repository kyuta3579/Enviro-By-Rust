#!/bin/sh
# docker build --platform linux/amd64 -t enviro-cross-rpz ./env/
docker run -it -v "$PWD":/Users/kohei/Documents/dev/enviro_play -e CARGO_HOME="$PWD"/.docker-cargo enviro-cross-rpz