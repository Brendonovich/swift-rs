use super::data::SRData;

#[derive(Debug)]
#[repr(C)]
pub struct SRString(SRData);

impl SRString {
    pub fn to_string(&self) -> String {
        unsafe { std::str::from_utf8_unchecked(self.0.into_slice()) }.into()
    }
}

extern "C" {
    pub fn allocate_string(data: *const u8, size: usize) -> &'static SRString;
}

impl From<&String> for &SRString {
    fn from(string: &String) -> &'static SRString {
        unsafe { allocate_string(string.as_ptr(), string.len()) }
    }
}

impl From<&str> for &SRString {
    fn from(string: &str) -> &'static SRString {
        unsafe { allocate_string(string.as_ptr(), string.len()) }
    }
}