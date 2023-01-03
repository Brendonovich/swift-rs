use serial_test::serial;
use std::process::Command;

#[test]
#[serial]
fn test_build() {
    let _ = Command::new("cargo")
        .arg("build")
        .current_dir("example")
        .output()
        .expect("failed to build example");
}

#[test]
#[serial]
fn test_run() {
    let _ = Command::new("cargo")
        .arg("run")
        .current_dir("example")
        .output()
        .expect("failed to build example");
}
