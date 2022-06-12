use swift_rs::{swift_fn, SRArray, SRObject, SRObjectArray, SRString};

#[repr(C)]
// #[swift_object]
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

// extern "C" {
//     fn get_file_thumbnail_base64(path: SRString) -> SRString;
//     fn get_mounts() -> SRObjectArray<Volume>;
//     fn return_nullable(null: bool) -> Option<SRObject<Test>>;
// }

swift_fn!(get_file_thumbnail_base64() -> SRString);

fn main() {
    let path = "/Users";
    let thumbnail = get_file_thumbnail_base64();
    std::fs::write("icon.txt", &thumbnail).unwrap();
    println!("Wrote folder icon base64 to icon.txt");

    // let mounts = unsafe { get_mounts() };
    // println!("First Volume Name: {}", mounts[0].name);

    // let opt = unsafe { return_nullable(true) };
    // println!("function returned nil: {}", opt.is_none());

    // let opt = unsafe { return_nullable(false) };
    // println!("function returned data: {}", opt.is_some());
}
