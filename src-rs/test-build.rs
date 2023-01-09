//! Build script for swift-rs that is a no-op for normal builds, but can be enabled
//! to include test swift library based on env var `TEST_SWIFT_RS=true` with the
//! `build` feature being enabled.

#[cfg(feature = "build")]
mod build;

fn main() {
    println!("cargo:rerun-if-env-changed={}", TEST_ENV);

    #[cfg(feature = "build")]
    if std::env::var(TEST_ENV).unwrap_or("false".into()) == "true" {
        build::link_swift("10.15");
        build::link_swift_package("test-swift", "tests/swift-pkg")
    }
}

const TEST_ENV: &str = "TEST_SWIFT_RS";
