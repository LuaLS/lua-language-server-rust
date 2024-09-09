#!/bin/bash

cargo build --release

if [ ! -d "dist" ]; then
    mkdir dist
fi

cp target/release/lua-language-server.exe dist/

cp -r resources dist/