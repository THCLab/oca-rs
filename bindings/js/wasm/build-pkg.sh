#!/bin/bash
OUTPUT_DIR=$1
if [ -z "$OUTPUT_DIR" ]
then
  OUTPUT_DIR="pkg"
fi

mkdir ./$OUTPUT_DIR
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen ./target/wasm32-unknown-unknown/release/oca_js.wasm --target nodejs --out-dir ./$OUTPUT_DIR/nodejs
wasm-bindgen ./target/wasm32-unknown-unknown/release/oca_js.wasm --target web --out-dir ./$OUTPUT_DIR/web

PACKAGE_VERSION=$(cat ./Cargo.toml \
  | grep version \
  | head -1 \
  | awk -F= '{ print $2 }' \
  | sed 's/"//g' \
  | tr -d '[[:space:]]')


cp ./pkg_templates/package.json ./$OUTPUT_DIR
sed -i "s/_VERSION_/$PACKAGE_VERSION/g" ./$OUTPUT_DIR/package.json
cp ./pkg_templates/README.md ./$OUTPUT_DIR
cp ./pkg_templates/gitignore ./$OUTPUT_DIR/.gitignore
