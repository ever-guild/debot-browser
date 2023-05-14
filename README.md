<div align="center">
  <h1><code>Debot browser</code></h1>
  <strong>A standalone console Debot browser written in Rust and compiled as WebAssembly</strong>
</div>

## How to use

Read this [readme](./debot-browser/README.md)

## How to build

### Prerequisites

For building WebAssembly:

- Rust stable >= 1.69.0
- [wasm-pack] tool
- Clang-8 (to successfully build zstd-sys)

[wasm-pack]: https://rustwasm.github.io/wasm-pack/installer/

For running test example:

- node >= 18

### 🛠️ Compile wasm

```shell
make debot-browser/pkg
```

### How to try

```bash
make start
```
