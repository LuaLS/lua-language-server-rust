#!/bin/bash

cargo build --release -p luals --features "no_format"

if [ -d "dist" ]; then
    rm -rf dist
fi

mkdir dist

cp target/release/lua-language-server dist/

cp -r resources dist/