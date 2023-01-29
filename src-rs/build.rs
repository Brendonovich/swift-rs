use std::{env, path::Path, process::Command};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwiftTargetInfo {
    pub triple: String,
    pub unversioned_triple: String,
    pub module_triple: String,
    //pub swift_runtime_compatibility_version: String,
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

struct RustTargetInfo {
    arch: String,
    os: String,
    sdk_name: String,
    simulator: bool,
}

fn get_target_info() -> RustTargetInfo {
    let mut arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if arch == "aarch64" {
        arch = "arm64".into();
    }
    let target = env::var("TARGET").unwrap();
    let os = match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "macos" => "macosx",
        "ios" => "ios",
        _ => panic!("unexpected target operating system"),
    };

    let simulator = target.ends_with("ios-sim");

    let sdk_name = if os == "macosx" {
        "macosx"
    } else if simulator {
        "iphonesimulator"
    } else {
        "iphoneos"
    };

    RustTargetInfo {
        arch,
        os: os.into(),
        sdk_name: sdk_name.into(),
        simulator,
    }
}

fn swift_target(
    target_info: &RustTargetInfo,
    minimum_macos_version: &'static str,
    minimum_ios_version: &'static str,
) -> String {
    format!(
        "{}-apple-{}{}{}",
        target_info.arch,
        target_info.os,
        if target_info.os == "ios" {
            minimum_ios_version
        } else {
            minimum_macos_version
        },
        // simulator suffix
        if target_info.simulator {
            "-simulator"
        } else {
            ""
        }
    )
}

pub fn get_swift_target_info(
    minimum_macos_version: &'static str,
    minimum_ios_version: &'static str,
) -> SwiftTarget {
    let target_info = get_target_info();
    let target = swift_target(&target_info, minimum_macos_version, minimum_ios_version);

    let swift_target_info_str = Command::new("swift")
        .args(&["-target", &target, "-print-target-info"])
        .output()
        .unwrap()
        .stdout;

    serde_json::from_slice(&swift_target_info_str).unwrap()
}

pub fn link_swift(minimum_macos_version: &'static str, minimum_ios_version: &'static str) {
    let swift_target_info = get_swift_target_info(minimum_macos_version, minimum_ios_version);

    swift_target_info
        .paths
        .runtime_library_paths
        .iter()
        .for_each(|path| {
            println!("cargo:rustc-link-search=native={}", path);
        });
}

pub fn link_swift_package(
    package_name: &str,
    package_root: &str,
    minimum_macos_version: &'static str,
    minimum_ios_version: &'static str,
) {
    let profile = env::var("PROFILE").unwrap();
    let package_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(package_root);

    let mut command = Command::new("swift");
    command
        .args(&["build", "-c", &profile])
        .current_dir(package_root);

    let target_info = get_target_info();

    if target_info.os == "ios" {
        let sdk_path_output = Command::new("xcrun")
            .args(["--sdk", &target_info.sdk_name, "--show-sdk-path"])
            .output()
            .unwrap();
        if !sdk_path_output.status.success() {
            panic!(
                "Failed to get SDK path with `xcrun --sdk {} --show-sdk-path`",
                target_info.sdk_name
            );
        }

        let sdk_path = String::from_utf8_lossy(&sdk_path_output.stdout);

        command.args([
            "-Xswiftc",
            "-sdk",
            "-Xswiftc",
            &sdk_path.trim(),
            "-Xswiftc",
            "-target",
            "-Xswiftc",
            &swift_target(&target_info, minimum_macos_version, minimum_ios_version),
        ]);
    }

    if !command.status().unwrap().success() {
        panic!("Failed to compile swift package {}", package_name);
    }

    let unversioned_triple = format!("{}-apple-macosx", target_info.arch);
    let search_path = package_path
        .join(".build")
        .join(unversioned_triple)
        .join(profile);

    // TODO: fix
    // println!(
    //     "cargo:rerun-if-changed={}",
    //     package_path.join("Sources").display()
    // );
    println!("cargo:rustc-link-search=native={}", search_path.display());
    println!("cargo:rustc-link-lib=static={}", package_name);
}
