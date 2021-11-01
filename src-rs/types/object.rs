use std::ops::Deref;

#[derive(Debug)]
#[repr(C)]
pub struct SRObject<T>(*const SRObjectImpl<T>);

#[derive(Debug)]
#[repr(C)]
struct SRObjectImpl<T> {
    _nsobject_offset: u8,
    data: T,
}

impl<T> Deref for SRObject<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(*self.0).data }
    }
}
