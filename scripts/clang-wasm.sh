#!/usr/bin/env sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
REPO_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"

exec clang \
  -I"$REPO_ROOT/wasm-sysroot" \
  -I"$REPO_ROOT/wasm-sysroot/src" \
  -I"$REPO_ROOT/wasm-sysroot/sys" \
  "$@"
