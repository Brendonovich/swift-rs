use std::ffi::c_void;

use crate::types::SRString;

extern "C" {
    pub fn release_object(obj: *const c_void);
    pub fn allocate_string(data: *const u8, size: usize) -> SRString;
}
