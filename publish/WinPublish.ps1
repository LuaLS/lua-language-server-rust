cargo build --release -p luals --features "no_format"

$distPath = "dist"

if (Test-Path -Path $distPath) {
    Remove-Item -Path $distPath -Recurse -Force
}

New-Item -ItemType Directory -Path $distPath

Copy-Item -Path "target/release/lua-language-server.exe" -Destination $distPath

Copy-Item -Path "resources" -Destination $distPath -Recurse