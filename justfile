set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

default:
    @just --list

wasm-build:
    @just wasm-build-{{ if os_family() == "windows" { "win" } else { "unix" } }}

wasm-build-win:
    @$env:CC_wasm32_unknown_unknown = (Resolve-Path scripts/clang-wasm.cmd).Path; wasm-pack build --target web --release

wasm-build-unix:
    @CC_wasm32_unknown_unknown="$PWD/scripts/clang-wasm.sh" wasm-pack build --target web --release

wasm-smoke:
    @node scripts/wasm-smoke.mjs
