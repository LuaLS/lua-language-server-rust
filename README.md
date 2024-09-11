# Project Status

This project is currently a work in progress. It is an exploration of using Rust as a host.

# Build Support

[x] win32-x64
[x] win32-ia32
[ ] linux-arm64 
[x] linux-x64
[x] linux-musl
[ ] linux-bsd
[x] darwin-x64
[x] darwin-arm64

# Build

Rust version: 1.81.0

To build the project, run:

```bash
git submodule update --init --recursive
cargo build
```

# Publish

To publish the project, run: 

```bash
./publish/WinPublish.ps1
```
will package the compiled files and related resource files into the `dist` directory
