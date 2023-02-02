use std::{ops::Deref, ptr::NonNull};

use crate::swift::SwiftObject;

use super::SRObject;

// SRArray is wrapped in SRObject since the
// Swift implementation extends NSObject
pub type SRObjectArray<T> = SRObject<SRArray<SRObject<T>>>;

#[repr(C)]
pub struct SRArrayImpl<T> {
    data: NonNull<T>,
    length: usize,
}

#[repr(transparent)]
pub struct SRArray<T>(SRObject<SRArrayImpl<T>>);

impl<T> SwiftObject for SRArray<T> {
    type Shape = SRArrayImpl<T>;

    fn get_object(&self) -> &SRObject<Self::Shape> {
        &self.0
    }
}

impl<T> Deref for SRArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T> SRArrayImpl<T> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data.as_ref(), self.length) }
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for SRArray<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}
