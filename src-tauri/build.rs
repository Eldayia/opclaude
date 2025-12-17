use std::path::Path;

fn main() {
    // Declare the custom cfg flag
    println!("cargo:rustc-check-cfg=cfg(no_frontend_dist)");

    // Check if frontend dist exists
    let dist_path = Path::new("../dist/index.html");
    if !dist_path.exists() {
        println!("cargo:warning=Frontend dist/index.html not found. Please run 'bun run build' first if you need web server mode.");
        // Create a flag to indicate dist is missing
        println!("cargo:rustc-cfg=no_frontend_dist");
    }

    tauri_build::build()
}
