# [![Rust Build Status]][Rust actions] [![Cargo version]][crates.io] [![WASM Build Status]][WASM actions] [![NPM version]][npmjs.com]

[Rust Build Status]: https://github.com/THCLab/oca-rust/actions/workflows/rust.yml/badge.svg?branch=main
[Rust actions]: https://github.com/THCLab/oca-rust/actions/workflows/rust.yml
[Cargo version]: https://img.shields.io/crates/v/oca-rust
[crates.io]: https://crates.io/crates/oca-rust
[WASM Build Status]: https://github.com/THCLab/oca-rust/actions/workflows/wasm.yml/badge.svg?branch=main
[WASM actions]: https://github.com/THCLab/oca-rust/actions/workflows/wasm.yml
[NPM version]: https://img.shields.io/npm/v/oca.js
[npmjs.com]: https://www.npmjs.com/package/oca.js
[Crates.io actions]: https://github.com/THCLab/oca-rust/actions/workflows/create.yml
[npmjs.com actions]: https://github.com/THCLab/oca-rust/actions/workflows/npm-publish.yml

[parser.bin release]: https://github.com/THCLab/oca-rust/releases/latest/download/parser.bin
[parser.exe release]: https://github.com/THCLab/oca-rust/releases/latest/download/parser.exe

# Rust implementation of Overlays Capture architecture

OCA is a standardized global solution for data capture and exchange which
protects PII data and provides a positive alternative to current architectures.
See more on: <https://oca.colossi.network/>

# Documentation

- [OCA Spec](https://the-human-colossus-foundation.github.io/oca-spec/)
- [API reference (docs.rs)](https://docs.rs/oca-rust)


# Usage

#### Command line parser

Download [bin for linux][parser.bin release] or [exe for windows][parser.exe release]

#### In cargo package

Add this to your `Cargo.toml`:

```toml
[dependencies]
oca-rust = "0.2.2"
```

### Build

Building local package with command line app and XLS parser:  
`cargo build --features command_line,xls_parser`

### Run [tests](tests)

`cargo test --all-features`

## JS WASM bindings

### Build

Building local NPM package  
in bindings/js/wasm directory:  
`bash build-pkg.sh`  

### Run [tests](bindings/js/example/test)

Go to bindings/js/example directory and install dependencies  

```
yarn install
yarn test
```

## Releasing new version

`cargo release`  
bumps version and runs `git push` with `v{version}` tag added.
That triggers actions on github
([Crates.io][Crates.io actions] and [npmjs.com][npmjs.com actions])
which build and publish packages on [crates.io][crates.io] and [npmjs.com][npmjs.com].
