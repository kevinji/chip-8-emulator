#!/usr/bin/env bash
set -euo pipefail

wasm-pack build --out-dir ../web-src/pkg --target web emulator
