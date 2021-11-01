use crate::types::{SRString};

// Returning a class type returns a pointer to that type

extern "C" {
    pub fn deallocate_string(string: SRString);
    pub fn allocate_string(data: *const u8, size: usize) -> SRString;
}
