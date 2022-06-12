use crate::SRArray;

use super::SRObject;

use std::ops::Deref;

#[repr(transparent)]
pub struct SRData(pub(crate) SRObject<SRDataImpl>);

#[repr(transparent)]
pub(crate) struct SRDataImpl(SRArray<u8>);

impl SRData {
    pub(crate) fn __retain(&self) {
        self.0.__retain();
    }
    pub fn release(&self) {
        self.0.release();
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
