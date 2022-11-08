use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET")?;
    if !target.contains("pc-windows") {
        return Ok(());
    }

    if !target.contains("msvc") || !target.contains("x86_64") {
        return Err("expected 64-bit msvc".into());
    }

    let root = env::var("CARGO_MANIFEST_DIR")?;
    let deps = PathBuf::from_iter([root.as_str(), "msvc", "x64"]);

    println!("cargo:rustc-link-search=all={}", deps.display());
    for entry in fs::read_dir(deps)? {
        let path = entry?.path();
        if let Some(filename) = path.file_name() {
            let filename = filename.to_str().unwrap();
            if !filename.ends_with(".dll") {
                continue;
            }
            let dest = PathBuf::from_iter([root.as_str(), filename]);
            fs::copy(&path, dest.as_path())?;
        }
    }

    Ok(())
}
