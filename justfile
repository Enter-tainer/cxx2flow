set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

wasi_sdk_path := justfile_directory() / "wasi-sdk"

default:
    @just --list

wasm-build:
    @just wasm-build-{{ if os_family() == "windows" { "win" } else { "unix" } }}

wasm-build-win:
    $env:WASI_SDK_PATH = "{{ wasi_sdk_path }}"; cargo build --lib --release --target wasm32-wasip1

wasm-build-unix:
    WASI_SDK_PATH="{{ wasi_sdk_path }}" cargo build --lib --release --target wasm32-wasip1

web-wasm:
    @just web-wasm-{{ if os_family() == "windows" { "win" } else { "unix" } }}

web-wasm-win:
    $env:WASI_SDK_PATH = "{{ wasi_sdk_path }}"; cargo build --lib --release --target wasm32-wasip1
    New-Item -ItemType Directory -Force web/src/wasm | Out-Null
    Copy-Item -Force target/wasm32-wasip1/release/cxx2flow_lib.wasm web/src/wasm/cxx2flow_bg.wasm

web-wasm-unix:
    WASI_SDK_PATH="{{ wasi_sdk_path }}" cargo build --lib --release --target wasm32-wasip1
    mkdir -p web/src/wasm
    cp target/wasm32-wasip1/release/cxx2flow_lib.wasm web/src/wasm/cxx2flow_bg.wasm

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
