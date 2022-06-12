use super::{array::SRArray, SRObject};

use std::ops::Deref;

#[repr(transparent)]
pub struct SRData(SRObject<SRDataImpl>);

#[repr(transparent)]
struct SRDataImpl(SRArray<u8>);

impl SRData {
    pub(crate) fn retain(&self) {
        self.0.retain();
    }
}

impl Deref for SRData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &*self.0.deref().0
    }
}

impl AsRef<[u8]> for SRData {
    fn as_ref(&self) -> &[u8] {
        &*self
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
