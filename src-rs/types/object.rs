use crate::swift;
use std::{ffi::c_void, ops::Deref, ptr::NonNull};

#[repr(transparent)]
pub struct SRObject<T>(NonNull<SRObjectImpl<T>>);

#[repr(C)]
struct SRObjectImpl<T> {
    _nsobject_offset: u8,
    data: T,
}

crate::swift::impl_to_swift!(SRObject<T>);

impl<T> Deref for SRObject<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &self.0.as_ref().data }
    }
}

impl<T> AsRef<T> for SRObject<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> Drop for SRObject<T> {
    fn drop(&mut self) {
        unsafe { swift::release_object(self.0.as_ref() as *const _ as *const c_void) }
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for SRObject<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.deref().serialize(serializer)
    }
}
