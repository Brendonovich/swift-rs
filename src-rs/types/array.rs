use std::ops::Deref;

use super::SRObject;

// SRArray is wrapped in SRObject since the
// Swift implementation extends NSObject
pub type SRObjectArray<T> = SRObject<SRArray<SRObject<T>>>;

#[derive(Debug)]
#[repr(C)]
pub struct SRArray<T>(SRObject<SRArrayImpl<T>>);

#[derive(Debug)]
#[repr(C)]
pub struct SRArrayImpl<T> {
    data: *const T,
    length: usize,
}

impl<T> Deref for SRArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let inner = &*self.0;
            std::slice::from_raw_parts(inner.data, inner.length)
        }
    }
}
