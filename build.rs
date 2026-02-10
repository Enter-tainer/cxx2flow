use anyhow::Result;
use vergen_gitcl::{BuildBuilder, CargoBuilder, Emitter, GitclBuilder, RustcBuilder};

fn main() -> Result<()> {
    // With WASI target, we get libc from wasi-sdk, no custom sysroot needed

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
