#![allow(dead_code)]
use std::{env, fmt::Display, path::Path, path::PathBuf, process::Command};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwiftTarget {
    triple: String,
    unversioned_triple: String,
    module_triple: String,
    //pub swift_runtime_compatibility_version: String,
    #[serde(rename = "librariesRequireRPath")]
    libraries_require_rpath: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwiftPaths {
    runtime_library_paths: Vec<String>,
    runtime_library_import_paths: Vec<String>,
    runtime_resource_path: String,
}

#[derive(Deserialize)]
struct SwiftEnv {
    target: SwiftTarget,
    paths: SwiftPaths,
}

impl SwiftEnv {
    fn new(minimum_macos_version: &str, minimum_ios_version: Option<&str>) -> Self {
        let rust_target = RustTarget::from_env();
        let target = rust_target.swift_target_triple(minimum_macos_version, minimum_ios_version);

        let swift_target_info_str = Command::new("swift")
            .args(["-target", &target, "-print-target-info"])
            .output()
            .unwrap()
            .stdout;

        serde_json::from_slice(&swift_target_info_str).unwrap()
    }
}

#[allow(clippy::upper_case_acronyms)]
enum RustTargetOS {
    MacOS,
    IOS,
}

impl RustTargetOS {
    fn from_env() -> Self {
        match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
            "macos" => RustTargetOS::MacOS,
            "ios" => RustTargetOS::IOS,
            _ => panic!("unexpected target operating system"),
        }
    }

    fn to_swift(&self) -> &'static str {
        match self {
            Self::MacOS => "macosx",
            Self::IOS => "ios",
        }
    }
}

impl Display for RustTargetOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MacOS => write!(f, "macos"),
            Self::IOS => write!(f, "ios"),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
enum SwiftSDK {
    MacOS,
    IOS,
    IOSSimulator,
}

impl SwiftSDK {
    fn from_os(os: &RustTargetOS) -> Self {
        let target = env::var("TARGET").unwrap();
        let simulator = target.ends_with("ios-sim");

        match os {
            RustTargetOS::MacOS => Self::MacOS,
            RustTargetOS::IOS if simulator => Self::IOSSimulator,
            RustTargetOS::IOS => Self::IOS,
        }
    }
}

impl Display for SwiftSDK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MacOS => write!(f, "macosx"),
            Self::IOSSimulator => write!(f, "iphonesimulator"),
            Self::IOS => write!(f, "iphoneos"),
        }
    }
}

struct RustTarget {
    arch: String,
    os: RustTargetOS,
    sdk: SwiftSDK,
}

impl RustTarget {
    fn from_env() -> Self {
        let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        let os = RustTargetOS::from_env();
        let sdk = SwiftSDK::from_os(&os);

        Self { arch, os, sdk }
    }

    fn swift_target_triple(
        &self,
        minimum_macos_version: &str,
        minimum_ios_version: Option<&str>,
    ) -> String {
        let unversioned = self.unversioned_swift_target_triple();
        format!(
            "{unversioned}{}{}",
            match (&self.os, minimum_ios_version) {
                (RustTargetOS::MacOS, _) => minimum_macos_version,
                (RustTargetOS::IOS, Some(version)) => version,
                _ => "",
            },
            // simulator suffix
            matches!(self.sdk, SwiftSDK::IOSSimulator)
                .then(|| "-simulator".to_string())
                .unwrap_or_default()
        )
    }

    fn unversioned_swift_target_triple(&self) -> String {
        format!(
            "{}-apple-{}",
            match self.arch.as_str() {
                "aarch64" => "arm64",
                a => a,
            },
            self.os.to_swift(),
        )
    }
}

struct SwiftPackage {
    name: String,
    path: PathBuf,
}

pub struct SwiftLinker {
    packages: Vec<SwiftPackage>,
    macos_min_version: String,
    ios_min_version: Option<String>,
}

impl SwiftLinker {
    pub fn new(macos_min_version: &str) -> Self {
        Self {
            packages: vec![],
            macos_min_version: macos_min_version.to_string(),
            ios_min_version: None,
        }
    }

    pub fn with_ios(mut self, min_version: &str) -> Self {
        self.ios_min_version = Some(min_version.to_string());
        self
    }

    pub fn with_package(mut self, name: &str, path: impl AsRef<Path>) -> Self {
        self.packages.extend([SwiftPackage {
            name: name.to_string(),
            path: path.as_ref().into(),
        }]);

        self
    }

    pub fn link(self) {
        let swift_env = SwiftEnv::new(&self.macos_min_version, self.ios_min_version.as_deref());

        #[allow(clippy::uninlined_format_args)]
        for path in swift_env.paths.runtime_library_paths {
            println!("cargo:rustc-link-search=native={path}");
        }

        let debug = env::var("DEBUG").unwrap() == "true";
        let configuration = if debug { "debug" } else { "release" };
        let rust_target = RustTarget::from_env();

        for package in self.packages {
            let package_path =
                Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(&package.path);

            let mut command = Command::new("swift");
            command
                .args(["build", "-c", &configuration])
                .current_dir(&package.path);

            if matches!(rust_target.os, RustTargetOS::IOS) {
                let sdk_path_output = Command::new("xcrun")
                    .args(["--sdk", &rust_target.sdk.to_string(), "--show-sdk-path"])
                    .output()
                    .unwrap();
                if !sdk_path_output.status.success() {
                    panic!(
                        "Failed to get SDK path with `xcrun --sdk {} --show-sdk-path`",
                        rust_target.sdk
                    );
                }

                let sdk_path = String::from_utf8_lossy(&sdk_path_output.stdout);

                command.args([
                    "-Xswiftc",
                    "-sdk",
                    "-Xswiftc",
                    sdk_path.trim(),
                    "-Xswiftc",
                    "-target",
                    "-Xswiftc",
                    &rust_target.swift_target_triple(
                        &self.macos_min_version,
                        self.ios_min_version.as_deref(),
                    ),
                ]);
            }

            if !command.status().unwrap().success() {
                panic!("Failed to compile swift package {}", package.name);
            }

            let search_path = package_path
                .join(".build")
                // swift build uses this output folder no matter what is the target
                .join(format!(
                    "{}-apple-macosx",
                    match std::env::consts::ARCH {
                        "aarch64" => "arm64",
                        arch => arch,
                    }
                ))
                .join(&configuration);

            // TODO: fix
            // println!(
            //     "cargo:rerun-if-changed={}",
            //     package_path.join("Sources").display()
            // );
            println!("cargo:rustc-link-search=native={}", search_path.display());
            println!("cargo:rustc-link-lib=static={}", package.name);
        }
    }
}
