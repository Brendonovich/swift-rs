use swift_rs::types::SRString;

extern "C" {
    fn get_file_thumbnail_base64(path: &SRString) -> &'static SRString;
}

fn main() {
    let thumbnail_raw = unsafe {
        get_file_thumbnail_base64("./main.rs".into())
    };
    
    let thumbnail = thumbnail_raw.to_string();
    
    println!("{}", thumbnail);
}
