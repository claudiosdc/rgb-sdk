#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null && pwd)"
PROJECT_DIR="$SCRIPT_DIR/.."

# Make sure that output directories exist
LIB_DIR="$PROJECT_DIR/lib"
INCLUDE_DIR="$PROJECT_DIR/include"

if [[ ! -d "$LIB_DIR" ]]; then
  mkdir "$LIB_DIR"
fi

if [[ ! -d "$LIB_DIR/linux" ]]; then
  mkdir "$LIB_DIR/linux"
fi

if [[ ! -d "$INCLUDE_DIR" ]]; then
  mkdir "$INCLUDE_DIR"
fi

# Build RGB SKD library
RUST_PROJECT_DIR="$PROJECT_DIR/../../librgb"

if [[ ! -d "$RUST_PROJECT_DIR" ]]; then
  echo "RGB SDK (Rust) project directory ($RUST_PROJECT_DIR) not found; aborting build"
  exit -1
fi

cargo build --target x86_64-unknown-linux-gnu --release --manifest-path "$RUST_PROJECT_DIR/Cargo.toml" && rsync -t "$RUST_PROJECT_DIR/target/x86_64-unknown-linux-gnu/release/librgb.so" "$LIB_DIR"/linux/ && rsync -t "$RUST_PROJECT_DIR/librgb.h" "$INCLUDE_DIR"