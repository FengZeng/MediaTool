// use std::fs;
use std::io;
// use std::path::Path;
// use std::path::PathBuf;

fn main() -> io::Result<()> {
    // let lib_path = PathBuf::from("libs");

    // let source_dir = Path::new("libs");
    // let target_dir = Path::new("target/debug/deps");

    // // Ensure the target directory exists
    // fs::create_dir_all(target_dir)?;

    // // Iterate over files in the source directory
    // for entry in fs::read_dir(source_dir)? {
    //     let entry = entry?;
    //     let entry_path = entry.path();

    //     // Check if the file is a .dylib
    //     if entry_path.extension().and_then(|ext| ext.to_str()) == Some("dylib") {
    //         // Create the target file path
    //         let target_path = target_dir.join(entry_path.file_name().unwrap());

    //         // Copy the file to the target directory
    //         fs::copy(&entry_path, &target_path)?;
    //         println!("Copied: {}", entry_path.display());
    //     }
    // }

    // println!("cargo:rustc-link-search=native={}", lib_path.display());
    //println!("cargo:rustc-link-lib=MediaTool");
    tauri_build::build();

    Ok(())
}
