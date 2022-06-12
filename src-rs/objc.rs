use std::ffi::c_void;

extern "C" {    
    pub(crate) fn objc_retain(obj: *const c_void);
    pub(crate) fn objc_release(obj: *const c_void);
}
