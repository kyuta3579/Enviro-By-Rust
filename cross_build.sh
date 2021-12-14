#!/bin/sh
# docker build -t enviro-cross-rpz ./env/
docker run -it --rm -v "$PWD":/Enviro-By-Rust -e CARGO_HOME=/Enviro-By-Rust/.docker-cargo enviro-cross-rpz