use swift_rs::*;

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

#[repr(C)]
struct Test {
    pub null: bool,
}

swift!(fn get_file_thumbnail_base64(path: &SRString) -> SRString);
swift!(fn get_mounts() -> SRObjectArray<Volume>);
swift!(fn return_nullable(null: bool) -> Option<SRObject<Test>>);

fn main() {
    let path = "/Users".into();
    let thumbnail = unsafe { get_file_thumbnail_base64(&path) };
    println!(
        "length of base64 encoded thumbnail: {}",
        thumbnail.as_str().len()
    );

    let mounts = unsafe { get_mounts() };
    println!("First Volume Name: {}", mounts[0].name);

    let opt = unsafe { return_nullable(true) };
    println!("function returned nil: {}", opt.is_none());

    let opt = unsafe { return_nullable(false) };
    println!("function returned data: {}", opt.is_some());
}
