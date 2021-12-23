use swift_rs::types::{SRObjectArray, SRString};

#[repr(C)]
struct Volume {
    pub name: SRString,
    path: SRString,
    total_capacity: usize,
    available_capacity: usize,
    is_removable: bool,
    is_ejectable: bool,
    is_root_filesystem: bool,
}

extern "C" {
    fn get_file_thumbnail_base64(path: SRString) -> SRString;
    fn get_mounts() -> SRObjectArray<Volume>;
}

fn main() {
    let path = "/Users";
    let thumbnail = unsafe { get_file_thumbnail_base64(path.into()) };
    std::fs::write("icon.txt", &thumbnail).unwrap();
    println!("Wrote folder icon base64 to icon.txt");
    
    let mounts = unsafe { get_mounts() };
    println!("First Volume Name: {}", mounts[0].name);
}
