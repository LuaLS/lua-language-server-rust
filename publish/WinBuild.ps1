cargo build --release -p luals

$distPath = "bin"

if (Test-Path -Path $distPath) {
    Remove-Item -Path $distPath -Recurse -Force
}

New-Item -ItemType Directory -Path $distPath

Copy-Item -Path "target/release/lua-language-server.exe" -Destination $distPath
