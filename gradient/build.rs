use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    if !target.contains("pc-windows") {
        return;
    }

    if !target.contains("msvc") || !target.contains("x86_64") {
        panic!("expected 64-bit msvc");
    }

    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let msvc = PathBuf::from_iter([root.as_str(), "msvc", "x64"]);

    println!("cargo:rustc-link-search=all={}", msvc.display());
    for entry in fs::read_dir(msvc).unwrap() {
        let path = entry.unwrap().path();
        if let Some(filename) = path.file_name() {
            let filename = filename.to_str().unwrap();
            if !filename.ends_with(".dll") {
                continue;
            }
            let dest = PathBuf::from_iter([root.as_str(), filename]);
            fs::copy(&path, dest.as_path()).unwrap();
        }
    }
}
