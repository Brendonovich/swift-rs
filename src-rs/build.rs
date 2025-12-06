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
    fn new(
        minimum_macos_version: &str,
        minimum_ios_version: Option<&str>,
        minimum_visionos_version: Option<&str>,
    ) -> Self {
        let rust_target = RustTarget::from_env();
        let target = rust_target.swift_target_triple(
            minimum_macos_version,
            minimum_ios_version,
            minimum_visionos_version,
        );

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
    VisionOS,
}

impl RustTargetOS {
    fn from_env() -> Self {
        match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
            "macos" => RustTargetOS::MacOS,
            "ios" => RustTargetOS::IOS,
            "visionos" => RustTargetOS::VisionOS,
            _ => panic!("unexpected target operating system"),
        }
    }

    fn to_swift(&self) -> &'static str {
        match self {
            Self::MacOS => "macosx",
            Self::IOS => "ios",
            Self::VisionOS => "xros",
        }
    }
}

impl Display for RustTargetOS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MacOS => write!(f, "macos"),
            Self::IOS => write!(f, "ios"),
            Self::VisionOS => write!(f, "visionos"),
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
enum SwiftSDK {
    MacOS,
    IOS,
    IOSSimulator,
    VisionOS,
    VisionOSSimulator,
}

impl SwiftSDK {
    fn from_os(os: &RustTargetOS) -> Self {
        let target = env::var("TARGET").unwrap();
        let simulator = target.ends_with("ios-sim")
            || target.ends_with("visionos-sim")
            || (target.starts_with("x86_64") && target.ends_with("ios"));

        match os {
            RustTargetOS::MacOS => Self::MacOS,
            RustTargetOS::IOS if simulator => Self::IOSSimulator,
            RustTargetOS::IOS => Self::IOS,
            RustTargetOS::VisionOS if simulator => Self::VisionOSSimulator,
            RustTargetOS::VisionOS => Self::VisionOS,
        }
    }

    fn clang_lib_extension(&self) -> &'static str {
        match self {
            Self::MacOS => "osx",
            Self::IOS => "ios",
            Self::IOSSimulator => "iossim",
            Self::VisionOS => "xros",
            Self::VisionOSSimulator => "xrsimulator",
        }
    }
}

impl Display for SwiftSDK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MacOS => write!(f, "macosx"),
            Self::IOSSimulator => write!(f, "iphonesimulator"),
            Self::IOS => write!(f, "iphoneos"),
            Self::VisionOSSimulator => write!(f, "xrsimulator"),
            Self::VisionOS => write!(f, "xros"),
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
        minimum_visionos_version: Option<&str>,
    ) -> String {
        let unversioned = self.unversioned_swift_target_triple();
        format!(
            "{unversioned}{}{}",
            match &self.os {
                RustTargetOS::MacOS => minimum_macos_version,
                RustTargetOS::IOS => minimum_ios_version.unwrap(),
                RustTargetOS::VisionOS => minimum_visionos_version.unwrap(),
            },
            // simulator suffix
            matches!(
                self.sdk,
                SwiftSDK::IOSSimulator | SwiftSDK::VisionOSSimulator
            )
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

/// Builder for linking the Swift runtime and custom packages.
#[cfg(feature = "build")]
pub struct SwiftLinker {
    packages: Vec<SwiftPackage>,
    /// Binary framework dependencies (e.g., xcframeworks) that need to be linked.
    frameworks: Vec<String>,
    macos_min_version: String,
    ios_min_version: Option<String>,
    visionos_min_version: Option<String>,
}

impl SwiftLinker {
    /// Creates a new [`SwiftLinker`] with a minimum macOS verison.
    ///
    /// Minimum macOS version must be at least 10.13.
    pub fn new(macos_min_version: &str) -> Self {
        Self {
            packages: vec![],
            frameworks: vec![],
            macos_min_version: macos_min_version.to_string(),
            ios_min_version: None,
            visionos_min_version: None,
        }
    }

    /// Instructs the [`SwiftLinker`] to also compile for iOS
    /// using the specified minimum iOS version.
    ///
    /// Minimum iOS version must be at least 11.
    pub fn with_ios(mut self, min_version: &str) -> Self {
        self.ios_min_version = Some(min_version.to_string());
        self
    }

    /// Instructs the [`SwiftLinker`] to also compile for visionOS
    /// using the specified minimum visionOS version.
    ///
    /// Minimum visionOS version must be at least 11.
    pub fn with_visionos(mut self, min_version: &str) -> Self {
        self.visionos_min_version = Some(min_version.to_string());
        self
    }

    /// Adds a package to be linked against.
    /// `name` should match the `name` field in your `Package.swift`,
    /// and `path` should point to the root of your Swift package relative
    /// to your crate's root.
    pub fn with_package(mut self, name: &str, path: impl AsRef<Path>) -> Self {
        self.packages.extend([SwiftPackage {
            name: name.to_string(),
            path: path.as_ref().into(),
        }]);

        self
    }

    /// Adds a binary framework dependency to be linked.
    ///
    /// Use this for xcframework dependencies that are downloaded by SPM
    /// (e.g., from a Swift package registry). The framework will be linked
    /// and an rpath will be set so it can be found at runtime.
    ///
    /// `name` should be the framework name without the `.framework` extension.
    pub fn with_framework(mut self, name: &str) -> Self {
        self.frameworks.push(name.to_string());
        self
    }

    /// Links the Swift runtime, then builds and links the provided packages.
    /// This does not (yet) automatically rebuild your Swift files when they are modified,
    /// you'll need to modify/save your `build.rs` file for that.
    pub fn link(self) {
        let swift_env = SwiftEnv::new(
            &self.macos_min_version,
            self.ios_min_version.as_deref(),
            self.visionos_min_version.as_deref(),
        );

        #[allow(clippy::uninlined_format_args)]
        for path in swift_env.paths.runtime_library_paths {
            println!("cargo:rustc-link-search=native={path}");
        }

        let debug = env::var("DEBUG").unwrap() == "true";
        let configuration = if debug { "debug" } else { "release" };
        let rust_target = RustTarget::from_env();

        link_clang_rt(&rust_target);

        for package in self.packages {
            let package_path =
                Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(&package.path);
            let out_path = Path::new(&env::var("OUT_DIR").unwrap())
                .join("swift-rs")
                .join(&package.name);

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

            let mut command = Command::new("swift");
            command.current_dir(&package.path);

            let arch = match std::env::consts::ARCH {
                "aarch64" => "arm64",
                arch => arch,
            };

            let swift_target_triple = rust_target.swift_target_triple(
                &self.macos_min_version,
                self.ios_min_version.as_deref(),
                self.visionos_min_version.as_deref(),
            );

            // swift build outputs here regardless of target platform
            let search_path = out_path
                .join(format!("{}-apple-macosx", arch))
                .join(configuration);

            command
                // Build the package (duh)
                .arg("build")
                // SDK path for regular compilation (idk)
                .args(["--sdk", sdk_path.trim()])
                // Release/Debug configuration
                .args(["-c", configuration])
                .args(["--arch", arch])
                // Where the artifacts will be generated to
                .args(["--build-path", &out_path.display().to_string()])
                // Override SDK path for each swiftc instance.
                // Necessary for iOS compilation.
                .args(["-Xswiftc", "-sdk"])
                .args(["-Xswiftc", sdk_path.trim()])
                // Override target triple for each swiftc instance.
                // Necessary for iOS compilation.
                .args(["-Xswiftc", "-target"])
                .args(["-Xswiftc", &swift_target_triple])
                .args(["-Xcc", &format!("--target={swift_target_triple}")])
                .args(["-Xcxx", &format!("--target={swift_target_triple}")])
                // Framework search path for packages that produce or depend on frameworks.
                // This matches the path used for cargo:rustc-link-search below.
                .args(["-Xswiftc", "-F"])
                .args(["-Xswiftc", &search_path.display().to_string()]);

            println!("Command `{command:?}`");

            if !command.status().unwrap().success() {
                panic!("Failed to compile swift package {}", package.name);
            }

            println!("cargo:rerun-if-changed={}", package_path.display());
            println!("cargo:rustc-link-search=native={}", search_path.display());

            // Auto-detect library type: check if static (.a) or dynamic (.dylib) was built
            let static_lib = search_path.join(format!("lib{}.a", package.name));
            let dynamic_lib = search_path.join(format!("lib{}.dylib", package.name));

            if dynamic_lib.exists() {
                println!("cargo:rustc-link-lib=dylib={}", package.name);
                println!("cargo:rustc-link-arg=-Wl,-rpath,{}", search_path.display());
            } else if static_lib.exists() {
                println!("cargo:rustc-link-lib=static={}", package.name);
            } else {
                panic!(
                    "Could not find built library for package '{}'. Expected either {} or {}",
                    package.name,
                    static_lib.display(),
                    dynamic_lib.display()
                );
            }

            // Link binary framework dependencies (xcframeworks downloaded by SPM)
            if !self.frameworks.is_empty() {
                println!(
                    "cargo:rustc-link-search=framework={}",
                    search_path.display()
                );
                println!("cargo:rustc-link-arg=-Wl,-rpath,{}", search_path.display());
                // Emit metadata so downstream crates can propagate the rpath.
                // Downstream crates can read this via DEP_<LINKS_NAME>_FRAMEWORK_PATH
                // if the current crate has `links = "..."` in Cargo.toml.
                println!("cargo:framework_path={}", search_path.display());
                for framework in &self.frameworks {
                    println!("cargo:rustc-link-lib=framework={}", framework);
                }
            }
        }
    }
}

/// Links the framework rpath from an upstream swift-rs crate with auto-detection.
///
/// This function automatically detects whether to propagate the framework path
/// to downstream crates based on whether this crate has a `links` key in Cargo.toml:
/// - If `CARGO_MANIFEST_LINKS` is set → propagates `cargo:framework_path` for downstream crates
/// - If `CARGO_MANIFEST_LINKS` is not set → only sets rpath for this crate (final binary)
///
/// Call this in `build.rs` of crates that transitively depend on a swift-rs crate.
/// The upstream crate must have `links = "<link_name>"` in Cargo.toml, and the
/// upstream crate must be a **direct** dependency of this crate.
///
/// # Arguments
/// * `link_name` - The value of the `links` key from the upstream crate's Cargo.toml
///
/// # Example
///
/// ```rust,ignore
/// // In a crate that depends on `am2` (which has `links = "am2"`)
/// fn main() {
///     #[cfg(target_os = "macos")]
///     swift_rs::link_swift_framework("am2");
/// }
/// ```
#[cfg(feature = "build")]
pub fn link_swift_framework(link_name: &str) {
    // Auto-detect: if this crate has `links = "..."`, it can propagate to downstream.
    // CARGO_MANIFEST_LINKS is only set when the crate has a `links` key.
    let should_propagate = env::var("CARGO_MANIFEST_LINKS").is_ok();
    do_link_framework_rpath(link_name, should_propagate);
}

/// Propagates framework rpath from an upstream crate that uses swift-rs.
///
/// Call this in `build.rs` of crates that transitively depend on a swift-rs crate.
/// The upstream crate must have `links = "<link_name>"` in Cargo.toml.
///
/// # Arguments
/// * `link_name` - The value of the `links` key from the upstream crate's Cargo.toml
/// * `propagate` - If true, also emits `cargo:framework_path` so further downstream
///   crates can continue the chain. Requires this crate to also have a `links` key.
///
/// # Example
///
/// ```rust,ignore
/// // In a crate that depends on `am2` (which has `links = "am2"`)
/// fn main() {
///     #[cfg(target_os = "macos")]
///     swift_rs::propagate_framework_rpath("am2", false);
/// }
/// ```
#[cfg(feature = "build")]
pub fn propagate_framework_rpath(link_name: &str, propagate: bool) {
    do_link_framework_rpath(link_name, propagate);
}

#[cfg(feature = "build")]
fn do_link_framework_rpath(link_name: &str, propagate: bool) {
    let env_var = format!(
        "DEP_{}_FRAMEWORK_PATH",
        link_name.to_uppercase().replace('-', "_")
    );
    if let Ok(path) = env::var(&env_var) {
        if !path.is_empty() {
            println!("cargo:rustc-link-arg=-Wl,-rpath,{}", path);
            if propagate {
                println!("cargo:framework_path={}", path);
            }
        }
    }
}

fn link_clang_rt(rust_target: &RustTarget) {
    println!(
        "cargo:rustc-link-lib=clang_rt.{}",
        rust_target.sdk.clang_lib_extension()
    );
    println!("cargo:rustc-link-search={}", clang_link_search_path());
}

fn clang_link_search_path() -> String {
    let output = std::process::Command::new(
        std::env::var("SWIFT_RS_CLANG").unwrap_or_else(|_| "/usr/bin/clang".to_string()),
    )
    .arg("--print-search-dirs")
    .output()
    .unwrap();
    if !output.status.success() {
        panic!("Can't get search paths from clang");
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("libraries: =") {
            let path = line.split('=').nth(1).unwrap();
            return format!("{}/lib/darwin", path);
        }
    }
    panic!("clang is missing search paths");
}
