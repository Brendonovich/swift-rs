use swift_rs::SwiftLinker;

fn main() {
    // Ensure this matches the versions set in your `Package.swift` file.
    SwiftLinker::new("10.15")
        .with_ios("11")
        .with_visionos("1")
        .with_package("swift-lib", "./swift-lib/")
        .link();
}
