# Project Status

This project is currently a work in progress. It is an exploration of using Rust as a host.

# Build Support

- [x] win32-x64
- [x] win32-ia32
- [x] linux-aarch64  not format 
- [x] linux-x64
- [x] linux-musl
- [x] linux-bsd  not format 
- [x] darwin-x64
- [x] darwin-arm64

NOTE: The linux-aarch64 and linux-bsd are not format, because there are some build problems.

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
