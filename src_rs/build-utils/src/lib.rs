use std::{
    env::{self},
    process::Command,
};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwiftTargetInfo {
    pub triple: String,
    pub unversioned_triple: String,
    pub module_triple: String,
    pub swift_runtime_compatibility_version: String,
    #[serde(rename = "librariesRequireRPath")]
    pub libraries_require_rpath: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwiftPaths {
    pub runtime_library_paths: Vec<String>,
    pub runtime_library_import_paths: Vec<String>,
    pub runtime_resource_path: String,
}

#[derive(Debug, Deserialize)]
pub struct SwiftTarget {
    pub target: SwiftTargetInfo,
    pub paths: SwiftPaths,
}

const MACOS_TARGET_VERSION: &str = "11";

pub fn get_swift_target_info() -> SwiftTarget {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target = format!("{}-apple-macosx{}", arch, MACOS_TARGET_VERSION);

    let swift_target_info_str = Command::new("swift")
        .args(&["-target", &target, "-print-target-info"])
        .output()
        .unwrap()
        .stdout;

    serde_json::from_slice(&swift_target_info_str).unwrap()
}

pub fn link_swift() {
    let swift_target_info = get_swift_target_info();
    if swift_target_info.target.libraries_require_rpath {
        panic!("Libraries require RPath! Change minimum MacOS value to fix.")
    }

    swift_target_info
        .paths
        .runtime_library_paths
        .iter()
        .for_each(|path| {
            println!("cargo:rustc-link-search=native={}", path);
        });
}

pub fn link_swift_package(package_name: &str, package_root: &str) {
    let profile = env::var("PROFILE").unwrap();

    if !Command::new("swift")
        .args(&["build", "-c", &profile])
        .current_dir(package_root)
        .status()
        .unwrap()
        .success()
    {
        panic!("Failed to compile swift package {}", package_name);
    }
    
    let swift_target_info = get_swift_target_info();

    println!(
        "cargo:rustc-link-search=native={}.build/{}/{}",
        package_root, swift_target_info.target.unversioned_triple, profile
    );
    println!("cargo:rustc-link-lib=static={}", package_name);
}
