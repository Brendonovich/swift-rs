use super::array::SRArray;

#[derive(Debug)]
#[repr(C)]
pub struct SRData {
    _nsobject_offset: u8,
    data: *mut SRArray<u8>,
}

impl SRData {
    pub fn into_slice(&self) -> &'static [u8] {
        unsafe { (*self.data).into_slice() }
    }

    pub fn data(&self) -> &SRArray<u8> {
        unsafe { &*(self.data) }
    }
}