//! Regression test for build scripts leaving state in the Swift package's source directory.
//!
//! `swift build` writes `Package.resolved` (and sometimes `.swiftpm`) next to
//! `Package.swift`, no matter where `--build-path` points. Cargo forbids build
//! scripts from modifying anything outside `OUT_DIR`, so anything left behind
//! makes `cargo package`/`cargo publish` fail verification with
//! "Source directory was modified by build.rs during cargo publish".
//!
//! Needs to be run with the env var `TEST_SWIFT_RS=true`, so that the build
//! script has actually built the test swift package.

use std::{env, path::Path};

#[test]
fn swift_build_leaves_package_dir_untouched() {
    if env::var("TEST_SWIFT_RS").unwrap_or_else(|_| "false".into()) != "true" {
        // The build script didn't run `swift build`, so there is nothing to assert.
        return;
    }

    let tests_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");

    // `swift-pkg-remote` is the one that matters: SwiftPM only writes
    // Package.resolved when the graph has a remote dependency to pin.
    for package in ["swift-pkg", "swift-pkg-remote"] {
        for artifact in ["Package.resolved", ".swiftpm"] {
            let path = tests_dir.join(package).join(artifact);
            assert!(
                !path.exists(),
                "{} was left in the swift package source directory by build.rs, \
                 which breaks `cargo package`",
                path.display()
            );
        }
    }
}
