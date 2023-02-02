use crate::swift::SwiftObject;

use super::{array::SRArray, SRObject};

use std::ops::Deref;

type Data = SRArray<u8>;

#[repr(transparent)]
pub struct SRData(SRObject<Data>);

impl SwiftObject for SRData {
    type Shape = Data;

    fn get_object(&self) -> &SRObject<Self::Shape> {
        &self.0
    }
}

impl Deref for SRData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<[u8]> for SRData {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for SRData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(self)
    }
}
