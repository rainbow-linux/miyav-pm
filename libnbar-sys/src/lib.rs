extern crate libc;
use libc::FILE;

extern "C" {
    pub fn nbar_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut NbarArchiveT;
    pub fn nbar_fclose(handle: *mut NbarArchiveT);
}

#[repr(C)]
pub struct NbarArchiveHeader {
    pub _magic_left: libc::uint16_t,
    pub file_name_length_1: libc::uint8_t,
    pub file_name_length_2: libc::uint8_t,
    pub file_name_1: [libc::c_char; 256],
    pub file_name_2: [libc::c_char; 256],
    pub file_checksum_1: [libc::c_char; 64],
    pub file_checksum_2: [libc::c_char; 64],
    pub file_length_1: libc::uint64_t,
    pub file_length_2: libc::uint64_t,
    pub _magic_right: libc::uint16_t,
}

#[repr(C)]
pub struct NbarArchive {
    pub header: NbarArchiveHeader,
    pub ar_content_1: *mut FILE,
    pub ar_content_2: *mut FILE,
    pub ar_file: *mut FILE,
}

