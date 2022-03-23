use std::{
    fmt::{Display, Error, Formatter},
    ops::Deref,
};

use crate::swift;

use super::data::SRData;

#[repr(transparent)]
pub struct SRString(SRData);

impl Deref for SRString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl SRString {
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&*self.0) }
    }
}

impl AsRef<[u8]> for SRString {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<&str> for SRString {
    fn from(string: &str) -> SRString {
        unsafe { swift::allocate_string(string.as_ptr(), string.len()) }
    }
}

impl Display for SRString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.as_str().fmt(f)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for SRString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "serde")]
impl<'a> serde::Deserialize<'a> for SRString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let string = String::deserialize(deserializer)?;
        Ok(SRString::from(string.as_str()))
    }
}
