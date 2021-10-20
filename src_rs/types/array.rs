#[derive(Debug)]
#[repr(C)]
pub struct SRArray<T> {
    _nsobject_offset: u8,
    data: *mut T,
    pub length: usize,
}

impl<T> SRArray<T> {
    pub fn into_slice(&self) -> &'static [T] {
        unsafe { std::slice::from_raw_parts(self.data, self.length) }
    }
}