use crate::swift::{self, SwiftObject};

use super::{array::SRArray, SRObject};

use std::ops::Deref;

type Data = SRArray<u8>;

/// Convenience type for working with byte buffers,
/// analagous to `SRData` in Swift.
///
/// ```rust
/// use swift_rs::{swift, SRData};
///
/// swift!(fn get_data() -> SRData);
///
/// let data = unsafe { get_data() };
///
/// assert_eq!(data.as_ref(), &[1, 2, 3])
/// ```
/// [_corresponding Swift code_]()
#[repr(transparent)]
pub struct SRData(SRObject<Data>);

impl SRData {
    pub fn as_array(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }
}

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

impl From<&Vec<u8>> for SRData {
    fn from(value: &Vec<u8>) -> SRData {
        let data = value.as_slice();
        unsafe { swift::allocate_data(data.as_ptr(), data.len()) }
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
