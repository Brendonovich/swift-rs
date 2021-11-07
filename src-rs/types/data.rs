use serde::{Serialize, Serializer};

use super::{SRObject, array::SRArray};

use std::ops::Deref;

#[derive(Debug)]
#[repr(C)]
pub struct SRData(SRObject<SRDataImpl>);

#[derive(Debug)]
#[repr(C)]
struct SRDataImpl(SRArray<u8>);

impl Deref for SRData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &*self.0.0
    }
}

impl AsRef<[u8]> for SRData {
    fn as_ref(&self) -> &[u8] {
        &*self
    }
}

impl Serialize for SRData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self)
    }
}