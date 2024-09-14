extern crate libc;

use libc::{c_char, FILE};
use std::ffi::CString;
use std::ptr;

// Declare the FFI struct and functions from libnbar
#[repr(C)]
pub struct NbarArchive {
    pub header: libc::c_void,  // Dummy field for simplicity in this example
    pub ar_content_1: *mut FILE,
    pub ar_content_2: *mut FILE,
    pub ar_file: *mut FILE,
}

// Extern block to declare FFI functions from the libnbar library
extern "C" {
    pub fn nbar_fopen(filename: *const c_char, mode: *const c_char) -> *mut NbarArchive;
    pub fn nbar_fclose(handle: *mut NbarArchive);
}

// Rust wrapper for nbar_fopen
pub fn nbar_open(filename: &str, mode: &str) -> Option<*mut NbarArchive> {
    let c_filename = CString::new(filename).unwrap();
    let c_mode = CString::new(mode).unwrap();

    unsafe {
        let handle = nbar_fopen(c_filename.as_ptr(), c_mode.as_ptr());
        if handle.is_null() {
            None
        } else {
            Some(handle)
        }
    }
}

// Rust wrapper for nbar_fclose
pub fn nbar_close(handle: *mut NbarArchive) {
    unsafe {
        if !handle.is_null() {
            nbar_fclose(handle);
        }
    }
}

fn main() {
    // Open and close an nbar archive
    let filename = "archive.nbar";
    let mode = "rb";

    match nbar_open(filename, mode) {
        Some(handle) => {
            println!("Successfully opened archive");
            nbar_close(handle);
        }
        None => {
            eprintln!("Failed to open archive");
        }
    }
}
