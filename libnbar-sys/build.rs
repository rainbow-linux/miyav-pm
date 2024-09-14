use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let xmake_dir = "sys-libnbar/";

    let xmake_status = Command::new("xmake")
                    .current_dir(xmake_dir)
                    .status()
                    .expect("failed to run xmake");
    
    if !xmake_status.success() {
        panic!("xmake build failed");
    }

    let xmake_out_dir = format!("{}/build/linux/x86_64/release", xmake_dir);

    println!("cargo:rustc-link-search=native={}", xmake_out_dir);
    println!("cargo:rustc-link-lib=shared=libnbar");
}