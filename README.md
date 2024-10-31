# Lua Language Server Rust Port

This is a Rust port of the Lua Language Server. Not all code is implemented in Rust; only the host program has been rewritten in Rust, while some C code is still used. The main goal of this port is to ensure compatibility with more platforms.

# Runtime

The current default runtime is 5.4

# Build Support

- [x] win32-x64
- [x] win32-ia32
- [x] linux-aarch64  not format 
- [x] linux-x64
- [x] linux-musl
- [x] linux-bsd  not format.
- [x] darwin-x64
- [x] darwin-arm64

NOTE: 
1. The linux-aarch64 and linux-bsd are not format, because there are some build problems.

# Build

Rust version: 1.81.0

To build the project, run:

```bash
git submodule update --init --recursive
cargo build --release -p luals
```

# Publish

To build the project, run: 

On Windows:
```bash
./publish/WinBuild.ps1
```

On other systems:
```bash
./publish/UnixBuild.sh
```

