use swift_rs::*;

#[swift_object]
struct Volume {
    pub name: SRString,
    path: SRString,
    total_capacity: Int,
    available_capacity: Int,
    is_removable: Bool,
    is_ejectable: Bool,
    is_root_filesystem: Bool,
}

#[derive(Debug)]
#[swift_object]
struct Test {
    pub null: Bool,
    pub num: Int,
}

swift_fn!(get_file_thumbnail_base64(path: &str) -> String);
swift_fn!(return_nullable(null: Bool) -> Option<Test>);
swift_fn!(get_mounts() -> SRTypedArray<Volume>);

fn main() {
    let path = "/Users";
    let thumbnail = get_file_thumbnail_base64(path);
    std::fs::write("icon.txt", thumbnail.as_str()).unwrap();
    println!("Wrote folder icon base64 to icon.txt");

    let mounts = get_mounts();
    println!("First Volume Name: {}", mounts[0].name);

    let opt = return_nullable(true);
    println!(
        "function returned data: {:?}",
        opt.as_ref().map(|o| o.as_ref())
    );

    let opt = return_nullable(false);
    println!(
        "function returned data: {:?}",
        opt.as_ref().map(|o| o.as_ref())
    );
}
