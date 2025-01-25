use num_cpus;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cpp_src_dir = manifest_dir.join("cpp_src");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Create build directory if it doesn't exist
    let build_dir = out_dir.join("build");
    std::fs::create_dir_all(&build_dir).unwrap();

    // Configure CMake
    let cmake_status = Command::new("cmake")
        .arg(&cpp_src_dir)
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", out_dir.display()))
        //.arg("-DCMAKE_OSX_DEPLOYMENT_TARGET=15.0")
        .arg("-DBUILD_ZENLIB=ON")
        .arg("-DBUILD_ZLIB=ON")
        .arg("-DBUILD_SHARED_LIBS=OFF")
        .current_dir(&build_dir)
        .status()
        .expect("Failed to run CMake configuration");
    assert!(cmake_status.success());

    // Build with CMake
    let cmake_build_status = Command::new("cmake")
        .arg("--build")
        .arg(".")
        .arg("--")
        .arg(format!("-j{}", num_cpus::get()))
        .current_dir(&build_dir)
        .status()
        .expect("Failed to run CMake build");
    assert!(cmake_build_status.success());

    let cmake_install_status = Command::new("cmake")
        .arg("--install")
        .arg(".")
        .current_dir(&build_dir)
        .status()
        .expect("Failed to run CMake install");
    assert!(cmake_install_status.success());

    // Tell cargo to link the library.
    println!("cargo:rustc-link-search=native={}/lib", out_dir.display());
    println!("cargo:rustc-link-lib=static=mediainfo");
    println!("cargo:rustc-link-lib=static=zen");
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=mediainfo_wrap");
    println!("cargo:rustc-link-lib=c++");
    //println!("cargo:rustc-link-lib=static=curl");

    // Tell cargo to look for the header file
    println!("cargo:include={}/include", out_dir.display());

    println!("cargo:rerun-if-changed=cpp_src/MediaInfoLib");
    println!("cargo:rerun-if-changed=cpp_src/ZenLib");
    println!("cargo:rerun-if-changed=cpp_src/CMakeLists.txt");
    println!("cargo:rerun-if-changed=cpp_src/MediaInfoWrap.cpp");
}
