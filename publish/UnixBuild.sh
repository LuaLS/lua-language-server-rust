#!/bin/bash

cargo build --release -p luals

if [ -d "dist" ]; then
    rm -rf dist
fi

mkdir bin

cp target/release/lua-language-server bin/
