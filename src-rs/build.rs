#![allow(dead_code)]
use std::{env, fmt::Display, fs, path::Path, path::PathBuf, process::Command};

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
        let simulator = target.ends_with("ios-sim")
            || (target.starts_with("x86_64") && target.ends_with("ios"));

        match os {
            RustTargetOS::MacOS => Self::MacOS,
            RustTargetOS::IOS if simulator => Self::IOSSimulator,
            RustTargetOS::IOS => Self::IOS,
        }
    }

    fn clang_lib_extension(&self) -> &'static str {
        match self {
            Self::MacOS => "osx",
            Self::IOS => "ios",
            Self::IOSSimulator => "iossim",
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

/// State that `swift build` writes into the package's source directory,
/// no matter where `--build-path` points.
///
/// Cargo forbids build scripts from touching anything outside `OUT_DIR`, so
/// leaving these behind makes `cargo package`/`cargo publish` fail verification
/// with "Source directory was modified by build.rs during cargo publish".
const SWIFTPM_SOURCE_STATE: [&str; 2] = ["Package.resolved", ".swiftpm"];

enum Snapshot {
    /// Didn't exist before the build, so anything there afterwards is ours to remove.
    Missing,
    File(Vec<u8>),
    /// A directory (or something we can't read); left untouched either way.
    Preexisting,
}

/// Restores [`SWIFTPM_SOURCE_STATE`] when dropped, including while unwinding
/// from a failed build, so the package's source directory comes out of
/// `swift build` exactly as it went in.
struct SourceStateGuard {
    entries: Vec<(PathBuf, Snapshot)>,
}

impl SourceStateGuard {
    fn new(package_path: &Path) -> Self {
        let entries = SWIFTPM_SOURCE_STATE
            .iter()
            .map(|name| {
                let path = package_path.join(name);
                let snapshot = match fs::read(&path) {
                    Ok(contents) => Snapshot::File(contents),
                    Err(_) if path.exists() => Snapshot::Preexisting,
                    Err(_) => Snapshot::Missing,
                };
                (path, snapshot)
            })
            .collect();

        Self { entries }
    }
}

impl Drop for SourceStateGuard {
    fn drop(&mut self) {
        for (path, snapshot) in &self.entries {
            match snapshot {
                Snapshot::Missing => {
                    let _ = if path.is_dir() {
                        fs::remove_dir_all(path)
                    } else {
                        fs::remove_file(path)
                    };
                }
                Snapshot::File(contents) => {
                    if fs::read(path).ok().as_ref() != Some(contents) {
                        let _ = fs::write(path, contents);
                    }
                }
                Snapshot::Preexisting => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SourceStateGuard, SWIFTPM_SOURCE_STATE};
    use std::{fs, path::PathBuf};

    /// A scratch package directory, removed when the test finishes.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!("swift-rs-test-{name}"));
            let _ = fs::remove_dir_all(&path);
            fs::create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn removes_state_the_build_created() {
        let dir = TempDir::new("created");

        {
            let _guard = SourceStateGuard::new(&dir.0);
            fs::write(dir.0.join("Package.resolved"), b"generated").unwrap();
            fs::create_dir_all(dir.0.join(".swiftpm").join("configuration")).unwrap();
        }

        for artifact in SWIFTPM_SOURCE_STATE {
            assert!(!dir.0.join(artifact).exists(), "{artifact} was left behind");
        }
    }

    #[test]
    fn restores_a_checked_in_lockfile() {
        let dir = TempDir::new("checked-in");
        let resolved = dir.0.join("Package.resolved");
        fs::write(&resolved, b"original").unwrap();

        {
            let _guard = SourceStateGuard::new(&dir.0);
            fs::write(&resolved, b"rewritten by swift build").unwrap();
        }

        assert_eq!(fs::read(&resolved).unwrap(), b"original");
    }

    #[test]
    fn leaves_a_preexisting_directory_alone() {
        let dir = TempDir::new("preexisting");
        let swiftpm = dir.0.join(".swiftpm");
        fs::create_dir_all(&swiftpm).unwrap();
        fs::write(swiftpm.join("keep-me"), b"user data").unwrap();

        drop(SourceStateGuard::new(&dir.0));

        assert_eq!(fs::read(swiftpm.join("keep-me")).unwrap(), b"user data");
    }

    #[test]
    fn cleans_up_when_the_build_panics() {
        let dir = TempDir::new("panicking");
        let resolved = dir.0.join("Package.resolved");

        let result = std::panic::catch_unwind({
            let dir = dir.0.clone();
            move || {
                let _guard = SourceStateGuard::new(&dir);
                fs::write(dir.join("Package.resolved"), b"generated").unwrap();
                panic!("Failed to compile swift package");
            }
        });

        assert!(result.is_err());
        assert!(!resolved.exists(), "Package.resolved survived the panic");
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
    macos_min_version: String,
    ios_min_version: Option<String>,
}

impl SwiftLinker {
    /// Creates a new [`SwiftLinker`] with a minimum macOS verison.
    ///
    /// Minimum macOS version must be at least 10.13.
    pub fn new(macos_min_version: &str) -> Self {
        Self {
            packages: vec![],
            macos_min_version: macos_min_version.to_string(),
            ios_min_version: None,
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

    /// Links the Swift runtime, then builds and links the provided packages.
    /// This does not (yet) automatically rebuild your Swift files when they are modified,
    /// you'll need to modify/save your `build.rs` file for that.
    pub fn link(self) {
        let swift_env = SwiftEnv::new(&self.macos_min_version, self.ios_min_version.as_deref());

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

            let swift_target_triple = rust_target
                .swift_target_triple(&self.macos_min_version, self.ios_min_version.as_deref());

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
                .args(["-Xcxx", &format!("--target={swift_target_triple}")]);

            // `swift build` writes Package.resolved (and sometimes .swiftpm) into
            // the package source directory; undo that once the build is done.
            let _source_state_guard = SourceStateGuard::new(&package_path);

            if !command.status().unwrap().success() {
                panic!("Failed to compile swift package {}", package.name);
            }

            let search_path = out_path
                // swift build uses this output folder no matter what is the target
                .join(format!("{}-apple-macosx", arch))
                .join(configuration);

            println!("cargo:rerun-if-changed={}", package_path.display());
            println!("cargo:rustc-link-search=native={}", search_path.display());
            println!("cargo:rustc-link-lib=static={}", package.name);
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
