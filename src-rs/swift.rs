use std::ffi::c_void;

use crate::{types::SRString, SRObject};

#[must_use = "A Ref MUST be sent over to the Swift side"]
#[repr(transparent)]
pub struct SwiftRef<T: SwiftObject>(SRObject<T::Shape>);

impl<T: SwiftObject> From<&T> for SwiftRef<T> {
    fn from(value: &T) -> Self {
        Self(value.get_object().clone())
    }
}

pub trait SwiftObject {
    type Shape;

    fn get_object(&self) -> &SRObject<Self::Shape>;
}

extern "C" {
    pub(crate) fn release_object(obj: *const c_void);
    pub(crate) fn allocate_string(data: *const u8, size: usize) -> SRString;
}
