//! Locate the shared LLVM on macOS through the binary's own rpath.
//!
//! rustc loads its own `@rpath/libLLVM.dylib`, so LLVM must never be exposed to
//! it through `DYLD_LIBRARY_PATH`. An rpath on `cheeku` and its test binaries
//! resolves Chuchu's LLVM without affecting any other process.
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=LLVM_SYS_231_PREFIX");
    println!("cargo:rerun-if-changed=build.rs");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("macos") {
        return;
    }
    let Some(prefix) = env::var_os("LLVM_SYS_231_PREFIX") else {
        return;
    };
    let lib = PathBuf::from(prefix).join("lib");
    if lib.join("libLLVM.dylib").exists() {
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib.display());
    }
}
