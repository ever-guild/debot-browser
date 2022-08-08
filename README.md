<div align="center">

  <h1><code>Cli DeBot Browser</code></h1>

  <strong>A standalone console DeBot Browser written in Rust and compiled as WebAssembly.</strong>

</div>

## How to use

Read this [readme](./debot-browser/README.md)

## Prerequisites

For building WebAssembly:

- Rust stable >= 1.55.
- [wasm-pack] tool.
- Clang-8 (to successfully build zstd-sys)

[wasm-pack]: https://rustwasm.github.io/wasm-pack/installer/

For running test example:

- npm >= 7.22.0

## 🛠️ Build

```bash
cd builder
cargo run
```
## How to try

```bash
cd test-server
npm install
npm run start
```

