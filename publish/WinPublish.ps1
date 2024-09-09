cargo build --release

if (-Not (Test-Path -Path "dist")) {
    New-Item -ItemType Directory -Path "dist"
}

Copy-Item -Path "target/release/lua-language-server.exe" -Destination "dist/"

Copy-Item -Path "resources" -Destination "dist/" -Recurse