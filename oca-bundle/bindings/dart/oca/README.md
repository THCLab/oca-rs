# OCA Dart Bindings

## Requirements

```sh
cargo install flutter_rust_bridge_codegen cbindgen
```

If `stdbool.h` is missing:

```sh
export CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d' ' -f1 | rev)/include"
```

## Build

```sh
flutter_rust_bridge_codegen --rust-input src/api.rs --dart-output oca/lib/bridge_generated.dart
cargo build
```

## Test

```sh
pushd oca
dart test
popd
```
