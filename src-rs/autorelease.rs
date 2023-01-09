use std::ffi::c_void;

extern "C" {
    pub fn objc_autoreleasePoolPush() -> *mut c_void;
    pub fn objc_autoreleasePoolPop(context: *mut c_void);
}
