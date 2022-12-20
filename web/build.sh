#!/bin/sh

set -e
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen ../target/wasm32-unknown-unknown/release/utk_level_editor_web.wasm --out-dir dist --out-name utk-level-editor --target web

cp index.html index.js dist/
cp ../assets/* dist/

if type wasm-opt >/dev/null 2>&1; then
  wasm-opt -Oz -o dist/utk-level-editor_bg.wasm dist/utk-level-editor_bg.wasm
else
  echo "warning: wasm-opt not found"
fi
