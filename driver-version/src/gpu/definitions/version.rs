use core::slice;
use std::ffi::{c_char, c_int};

/// C definition for exchanging driver version information.
#[repr(C)]
#[derive(Debug)]
pub struct CDrmVersion {
    version_major: c_int,
    version_minor: c_int,
    version_patchlevel: c_int,
    name_len: usize,
    name: *mut c_char,
    date_len: usize,
    date: *mut c_char,
    desc_len: usize,
    desc: *mut c_char,
}

impl Default for CDrmVersion {
    fn default() -> Self {
        Self {
            version_major: 0,
            version_minor: 0,
            version_patchlevel: 0,
            name_len: 256,
            name: vec![0; 256].as_mut_ptr(),
            date_len: 256,
            date: vec![0; 256].as_mut_ptr(),
            desc_len: 256,
            desc: vec![0; 256].as_mut_ptr(),
        }
    }
}

/// Mirror structure of the kernel C driver version definition but with Rust
/// types.
pub struct DrmVersion<'c_struct_lifetime> {
    pub major: i32,
    pub minor: i32,
    pub patchlevel: i32,

    pub name: &'c_struct_lifetime str,
    pub date: &'c_struct_lifetime str,
    pub desc: &'c_struct_lifetime str,
}

/// Converts a kernel string composed of a data pointer and a length into a &str
/// slice with lifetime same as the underlying data pointer.
macro_rules! kernel_str {
    ($data:expr, $len:expr) => {
        unsafe { std::str::from_utf8(slice::from_raw_parts($data as *const u8, $len)).unwrap() }
    };
}

impl<'c_struct_lifetime> From<CDrmVersion> for DrmVersion<'c_struct_lifetime> {
    fn from(version: CDrmVersion) -> Self {
        Self {
            major: version.version_major,
            minor: version.version_minor,
            patchlevel: version.version_patchlevel,
            name: kernel_str!(version.name, version.name_len),
            date: kernel_str!(version.date, version.date_len),
            desc: kernel_str!(version.desc, version.desc_len),
        }
    }
}
