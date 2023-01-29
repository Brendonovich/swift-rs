use swift_rs::build;

fn main() {
    build::link_swift("10.15", "11"); // Ensure this matches the version set in your `Package.swift` file.
    build::link_swift_package("swift-lib", "./swift-lib/", "10.15", "11");
}
