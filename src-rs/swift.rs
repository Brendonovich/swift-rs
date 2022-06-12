use std::ffi::c_void;

use crate::types::SRString;

extern "C" {
    pub(crate) fn release_object(obj: *const c_void);
    pub(crate) fn retain_object(obj: *const c_void);
    pub(crate) fn allocate_string(data: *const u8, size: usize) -> SRString;
    
    pub(crate) fn objc_retainAutoreleaseReturnValue(obj: *const c_void) -> *const c_void;
}
