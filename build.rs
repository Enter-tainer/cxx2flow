use anyhow::Result;
use std::env;

#[cfg(not(target_arch = "wasm32"))]
use vergen::EmitBuilder;

fn main() -> Result<()> {
    // Emit the instructions - but only when vergen is available (non-WASM)
    let target = env::var("TARGET").unwrap_or_default();
    if !target.starts_with("wasm32") {
        #[cfg(not(target_arch = "wasm32"))]
        {
            EmitBuilder::builder()
                .all_cargo()
                .build_timestamp()
                .git_sha(false)
                .git_describe(true, true, None)
                .all_rustc()
                .emit()?;
        }
    }
    
    Ok(())
}
