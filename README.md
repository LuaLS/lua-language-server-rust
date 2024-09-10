# Project Status

This project is currently a work in progress. It is an exploration of using Rust as a host.

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
