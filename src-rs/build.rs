use std::{env, path::Path, process::Command};

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

pub fn get_swift_target_info(minimum_mac_os_version: &'static str) -> SwiftTarget {
    let mut arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if arch == "aarch64" {
        arch = "arm64".into();
    }
    let target = format!("{}-apple-macosx{}", arch, minimum_mac_os_version);

    let swift_target_info_str = Command::new("swift")
        .args(&["-target", &target, "-print-target-info"])
        .output()
        .unwrap()
        .stdout;

    serde_json::from_slice(&swift_target_info_str).unwrap()
}

pub fn link_swift(minimum_mac_os_version: &'static str) {
    let swift_target_info = get_swift_target_info(minimum_mac_os_version);

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
    let package_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(package_root);

    if !Command::new("swift")
        .args(&["build", "-c", &profile])
        .current_dir(&package_path)
        .status()
        .unwrap()
        .success()
    {
        panic!("Failed to compile swift package {}", package_name);
    }

    let mut arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if arch == "aarch64" {
        arch = "arm64".into();
    }
    let unversioned_triple = format!("{}-apple-macosx", arch);
    let search_path = package_path
        .join(".build")
        .join(unversioned_triple)
        .join(profile);

    println!("cargo:rustc-link-search=native={}", search_path.display());
    println!("cargo:rustc-link-lib=static={}", package_name);
}
