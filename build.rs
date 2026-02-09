use anyhow::Result;
use vergen::EmitBuilder;

fn main() -> Result<()> {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("wasm") {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
        let wasm_sysroot = std::path::Path::new(&manifest_dir).join("wasm-sysroot");

        let mut build = cc::Build::new();
        build
            .include(wasm_sysroot.join("src"))
            .include(&wasm_sysroot)
            .opt_level_str("z")
            .warnings(false)
            .target(&target)
            .host(&target);

        build.file(wasm_sysroot.join("src/stdlib.c"));
        build.file(wasm_sysroot.join("src/stdio.c"));
        build.file(wasm_sysroot.join("src/ctype.c"));
        build.file(wasm_sysroot.join("src/string.c"));
        build.compile("cxx2flow_wasm_sysroot");

        println!("cargo:rerun-if-changed={}", wasm_sysroot.display());
    }

    // Emit the instructions
    EmitBuilder::builder()
        .all_cargo()
        .build_timestamp()
        .git_sha(false)
        .git_describe(true, true, None)
        .all_rustc()
        .emit()?;
    Ok(())
}
