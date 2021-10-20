#[derive(Debug)]
#[repr(C)]
struct SRArray<T> {
    _nsobject_offset: u8,
    data: *mut T,
    length: usize,
}

impl<T> SRArray<T> {
    fn into_slice(&self) -> &'static [T] {
        unsafe { std::slice::from_raw_parts(self.data, self.length) }
    }
}

#[derive(Debug)]
#[repr(C)]
struct SRData {
    _nsobject_offset: u8,
    data: *mut SRArray<u8>,
}

impl SRData {
    fn into_slice(&self) -> &'static [u8] {
        unsafe { (*self.data).into_slice() }
    }

    fn data(&self) -> &SRArray<u8> {
        unsafe { &*(self.data) }
    }
}

#[derive(Debug)]
#[repr(C)]
struct SRString(SRData);

impl SRString {
    fn to_string(&self) -> String {
        unsafe { std::str::from_utf8_unchecked(self.0.into_slice()) }.into()
    }
}

impl From<&String> for &SRString {
    fn from(string: &String) -> &'static SRString {
        unsafe { allocate_string(string.as_ptr(), string.len()) }
    }
}

extern "C" {
    fn return_data() -> &'static SRData;
    fn return_string() -> &'static SRString;
    fn echo_string(string: &SRString);
    fn allocate_string(data: *const u8, size: usize) -> &'static SRString;
    #[link_name="get_file_thumbnail_base64"]
    fn get_file_thumbnail_base64_(path: &SRString) -> &'static SRString;
}

fn get_file_thumbnail_base64(path: &SRString) -> &'static SRString {
    unsafe { get_file_thumbnail_base64_(path) }
}

fn main() {
    let path = &std::env::var("FILE_PATH").unwrap();
    
    let thumbnail_b46 = get_file_thumbnail_base64(path.into()).to_string();

    std::fs::write("output.txt", &thumbnail_b46).unwrap();
}
