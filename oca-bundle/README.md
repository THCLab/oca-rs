# [![Rust Build Status]][Rust actions] [![Cargo version]][crates.io] [![WASM Build Status]][WASM actions] [![NPM version]][npmjs.com]

[Rust Build Status]: https://github.com/THCLab/oca-rs/actions/workflows/rust.yml/badge.svg?branch=main
[Rust actions]: https://github.com/THCLab/oca-rs/actions/workflows/rust.yml
[Cargo version]: https://img.shields.io/crates/v/oca-rs
[crates.io]: https://crates.io/crates/oca-rs
[WASM Build Status]: https://github.com/THCLab/oca-rs/actions/workflows/wasm.yml/badge.svg?branch=main
[WASM actions]: https://github.com/THCLab/oca-rs/actions/workflows/wasm.yml
[NPM version]: https://img.shields.io/npm/v/oca.js
[npmjs.com]: https://www.npmjs.com/package/oca.js
[Crates.io actions]: https://github.com/THCLab/oca-rs/actions/workflows/create.yml
[npmjs.com actions]: https://github.com/THCLab/oca-rs/actions/workflows/npm-publish.yml

# Rust implementation of Overlays Capture architecture

OCA is a standardized global solution for data capture and exchange which
protects PII data and provides a positive alternative to current architectures.
See more on: <https://oca.colossi.network/>

## License

EUPL 1.2 

We have distilled the most crucial license specifics to make your adoption seamless: [see here for details](https://github.com/THCLab/licensing).

# Documentation

- [OCA Spec](https://oca.colossi.network/)
- [API reference (docs.rs)](https://docs.rs/oca-rs)


# Usage

The MSRV is `1.58.1`

#### In cargo package

Add this to your `Cargo.toml`:

```toml
[dependencies]
oca-bundle = "0.4.4"
```

### Build

Building local package:  
`cargo build `

### Run [tests](tests)

`cargo test`

## Bindings

To use oca in other languages, checkout [oca-bindings](https://github.com/THCLab/oca-bindings).

## Releasing new version

`cargo release`  
bumps version and runs `git push` with `v{version}` tag added.
That triggers actions on github
([Crates.io][Crates.io actions] and [npmjs.com][npmjs.com actions])
which build and publish packages on [crates.io][crates.io] and [npmjs.com][npmjs.com].

# Contributing

See https://github.com/THCLab/contributing
