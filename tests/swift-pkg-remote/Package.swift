// swift-tools-version:5.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "test-swift-remote",
    platforms: [
        .macOS(.v11),
    ],
    products: [
        .library(
            name: "test-swift-remote",
            type: .static,
            targets: ["test-swift-remote"]),
    ],
    dependencies: [
        // Exists purely so that SwiftPM has something to pin: it only writes
        // Package.resolved into this directory when the graph contains a remote
        // dependency, and that file is what `cargo package` trips over. This is
        // swift-rs' own published package so no third party is involved, and the
        // target below deliberately doesn't use it, so nothing extra is compiled
        // or linked.
        .package(url: "https://github.com/Brendonovich/swift-rs", .exact("1.0.7"))
    ],
    targets: [
        .target(
            name: "test-swift-remote",
            dependencies: [],
            path: ".")
    ]
)
