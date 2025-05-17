# raw-window-handle-ffi

- C side: header-only type definitions for FFI with rust crate `raw-window-handle`.
- Rust side: FFI types meant to make `raw-window-handle` types FFI-compatible.

## Build & Install

AUR: TODO

Debian package: TODO

Windows installer: TODO

Linux installer: TODO

```bash
$ cargo install cbindgen
$ ~/.cargo/bin/cbindgen --config cbindgen.toml  --output raw-window-handle-ffi.h
$ sudo install -Dm644 /usr/include/raw-window-handle-ffi.h raw-window-handle-ffi.h
```
