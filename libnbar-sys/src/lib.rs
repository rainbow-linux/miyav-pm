extern crate libc;
use libc::FILE;

extern "C" {
    pub(crate) fn nbar_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut NbarArchive;
    pub(crate) fn nbar_fclose(handle: *mut NbarArchive);
}

#[repr(C)]
pub struct NbarArchiveHeader {
    pub _magic_left: u16,
    pub file_name_length_1: u8,
    pub file_name_length_2: u8,
    pub file_name_1: [libc::c_char; 256],
    pub file_name_2: [libc::c_char; 256],
    pub file_checksum_1: [libc::c_char; 64],
    pub file_checksum_2: [libc::c_char; 64],
    pub file_length_1: u64,
    pub file_length_2: u64,
    pub _magic_right:  u64,
}

#[repr(C)]
pub struct NbarArchive {
    pub header: NbarArchiveHeader,
    pub(crate) ar_content_1: *mut FILE,
    pub(crate) ar_content_2: *mut FILE,
    pub(crate) ar_file: *mut FILE,
}