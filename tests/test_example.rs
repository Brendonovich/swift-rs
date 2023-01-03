use serial_test::serial;
use std::{
    fs::{read_to_string, write},
    process::Command,
};

/// This test tries building the code in example/ directory
#[test]
#[serial]
fn test_build() {
    let status = Command::new("cargo")
        .args(["build", "-vv"])
        .current_dir("example")
        .status();
    assert!(
        status.expect("cmd failure").success(),
        "failed to build example"
    );
}

/// This test tries building the code in example/ directory, but after modifying
/// the `package_root` in `link_swift_package()` call to not have a trailing slash.
#[test]
#[serial]
fn test_link_swift_pkg_without_trailing_slash() {
    // modify build.rs of example
    let build_rs = "example/src/build.rs";
    let build_rs_code = read_to_string(build_rs).expect("failed to read build script");
    let new_code = build_rs_code.replace("./swift-lib/", "./swift-lib");
    write(build_rs, new_code).expect("failed to write to build script");

    let status = Command::new("cargo")
        .args(["build", "-vv"])
        .current_dir("example")
        .status();

    // replace build.rs to original code
    write(build_rs, build_rs_code).expect("failed to write to build script");

    // check the result of build output
    assert!(
        status.expect("cmd failure").success(),
        "failed to build example without trailing slash in pkg dir"
    );
}

/// This test runs the code in example/ directory.
///
/// TODO:this should be replaced with better & more structured tests
#[test]
#[serial]
fn test_run() {
    let status = Command::new("cargo")
        .arg("run")
        .current_dir("example")
        .status();
    assert!(
        status.expect("cmd failure").success(),
        "failed to run example"
    );
}
