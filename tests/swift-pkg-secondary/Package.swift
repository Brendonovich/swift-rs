// swift-tools-version:5.3

import PackageDescription

let package = Package(
    name: "test-swift-secondary",
    platforms: [
        .macOS(.v11),
    ],
    products: [
        .library(
            name: "test-swift-secondary",
            type: .static,
            targets: ["test-swift-secondary"]),
    ],
    dependencies: [
        .package(name: "SwiftRs", path: "../../")
    ],
    targets: [
        .target(
            name: "test-swift-secondary",
            dependencies: [.product(name: "SwiftRs", package: "SwiftRs")],
            path: ".")
    ]
)
