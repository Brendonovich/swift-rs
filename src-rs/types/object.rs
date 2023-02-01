use crate::{
    swift::{self, SwiftObject},
    SwiftRef,
};
use std::{ffi::c_void, ops::Deref, ptr::NonNull};

#[repr(C)]
pub struct SRObjectImpl<T> {
    _nsobject_offset: u8,
    data: T,
}

#[repr(transparent)]
pub struct SRObject<T>(NonNull<SRObjectImpl<T>>);

impl<T> SRObject<T> {
    pub(crate) fn clone(&self) -> Self {
        Self(self.0)
    }

    pub fn swift_ref(&self) -> SwiftRef<Self> {
        self.into()
    }
}

impl<T> SwiftObject for SRObject<T> {
    type Shape = T;

    fn get_object(&self) -> &SRObject<Self::Shape> {
        self
    }
}

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
