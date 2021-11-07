use std::ops::Deref;

use serde::{Deserialize, Serialize, ser::SerializeSeq};

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
        self.0.as_slice()
    }
}

impl<T> SRArrayImpl<T> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data, self.length) }
    }
}

impl<T> Serialize for SRArray<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}