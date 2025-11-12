use anyhow::Result;
use std::env;
use std::path::PathBuf;
use std::process::Command;

#[cfg(not(target_arch = "wasm32"))]
use vergen::EmitBuilder;

fn main() -> Result<()> {
    let target = env::var("TARGET").unwrap_or_default();
    
    // Emit build information for non-WASM builds
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
    } else {
        // For WASM builds, compile tree-sitter-cpp to WASM
        compile_tree_sitter_cpp_wasm()?;
    }
    
    Ok(())
}

fn compile_tree_sitter_cpp_wasm() -> Result<()> {
    println!("cargo:warning=Compiling tree-sitter-cpp grammar to WASM");
    
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    
    // Clone tree-sitter-cpp if not exists
    let cpp_dir = out_dir.join("tree-sitter-cpp");
    if !cpp_dir.exists() {
        println!("cargo:warning=Cloning tree-sitter-cpp repository...");
        let status = Command::new("git")
            .args(&[
                "clone",
                "--depth", "1",
                "https://github.com/tree-sitter/tree-sitter-cpp",
                cpp_dir.to_str().unwrap(),
            ])
            .status()?;
        
        if !status.success() {
            anyhow::bail!("Failed to clone tree-sitter-cpp");
        }
    }
    
    // Build WASM module using tree-sitter CLI
    let wasm_file = out_dir.join("tree-sitter-cpp.wasm");
    
    println!("cargo:warning=Building WASM grammar with tree-sitter CLI...");
    println!("cargo:warning=This requires Docker to be available and may take a few minutes on first build.");
    
    let output = Command::new("tree-sitter")
        .args(&[
            "build",
            "--wasm",
            "-o",
            wasm_file.to_str().unwrap(),
        ])
        .current_dir(&cpp_dir)
        .output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("cargo:warning=STDOUT: {}", stdout);
        println!("cargo:warning=STDERR: {}", stderr);
        anyhow::bail!("Failed to compile tree-sitter-cpp to WASM. Make sure tree-sitter CLI is installed and Docker is available.");
    }
    
    // Verify the WASM file was created
    if !wasm_file.exists() {
        anyhow::bail!("WASM file was not created at {}. tree-sitter build may have failed silently.", wasm_file.display());
    }
    
    println!("cargo:warning=WASM grammar built successfully at {}", wasm_file.display());
    println!("cargo:warning=WASM file size: {} bytes", wasm_file.metadata()?.len());
    
    // Tell cargo where to find the WASM file
    println!("cargo:rustc-env=TREE_SITTER_CPP_WASM_PATH={}", wasm_file.display());
    println!("cargo:rerun-if-changed={}", cpp_dir.display());
    
    Ok(())
}
