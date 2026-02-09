set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

default:
    @just --list

wasm-build:
    @just wasm-build-{{ if os_family() == "windows" { "win" } else { "unix" } }}

wasm-build-win:
    @$env:CC_wasm32_unknown_unknown = (Resolve-Path scripts/clang-wasm.cmd).Path; wasm-pack build --target web --release

wasm-build-unix:
    @CC_wasm32_unknown_unknown="$PWD/scripts/clang-wasm.sh" wasm-pack build --target web --release

web-wasm:
    @just web-wasm-{{ if os_family() == "windows" { "win" } else { "unix" } }}

web-wasm-win:
    @$env:CC_wasm32_unknown_unknown = (Resolve-Path scripts/clang-wasm.cmd).Path; wasm-pack build . --target web --release --out-dir pkg --out-name cxx2flow

web-wasm-unix:
    @CC_wasm32_unknown_unknown="$PWD/scripts/clang-wasm.sh" wasm-pack build . --target web --release --out-dir pkg --out-name cxx2flow

web-install:
    @cd web; pnpm install

web-dev:
    @just web-wasm
    @cd web; pnpm dev

web-build:
    @just web-wasm
    @cd web; pnpm build

wasm-smoke:
    @node scripts/wasm-smoke.mjs

test-snapshots:
    @cargo test --test snapshot_integration
