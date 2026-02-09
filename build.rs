use anyhow::Result;
use vergen_gitcl::{BuildBuilder, CargoBuilder, Emitter, GitclBuilder, RustcBuilder};

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

    let build = BuildBuilder::default().build_timestamp(true).build()?;
    let cargo = CargoBuilder::all_cargo()?;
    let rustc = RustcBuilder::all_rustc()?;
    let gitcl = GitclBuilder::default()
        .describe(true, true, None)
        .sha(false)
        .build()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .add_instructions(&gitcl)?
        .emit()?;
    Ok(())
}
