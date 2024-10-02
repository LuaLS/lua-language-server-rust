# Lua Language Server Rust Port

This is a Rust port of the Lua Language Server. Not all code is implemented in Rust; only the host program has been rewritten in Rust, while some C code is still used. The main goal of this port is to ensure compatibility with more platforms.

# Runtime

The current default runtime is LuaJIT, but JIT is not enabled. Performance is better with `jit.off()`.

# Build Support

- [x] win32-x64
- [x] win32-ia32
- [x] linux-aarch64  not format 
- [x] linux-x64
- [x] linux-musl
- [x] linux-bsd  not format. need 'ld' but not find.
- [x] darwin-x64
- [x] darwin-arm64

NOTE: 
1. The linux-aarch64 and linux-bsd are not format, because there are some build problems.

2. linux-bsd Unable to compile successfully under the virtual machine provided by cross, you can compile it yourself.

# Build

Rust version: 1.81.0

To build the project, run:

```bash
git submodule update --init --recursive
cargo build --release -p luals
```

# Publish

To publish the project, run: 

```bash
./publish/WinPublish.ps1
```
will package the compiled files and related resource files into the `dist` directory
