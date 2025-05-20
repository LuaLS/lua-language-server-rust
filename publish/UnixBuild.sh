#!/usr/bin/env bash

cargo build --release -p luals

if [ -d "dist" ]; then
    rm -rf dist
fi

mkdir -p  bin

cp target/release/lua-language-server bin/
