// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Day15",
    // products: [
    //     // Products define the executables and libraries a package produces, making them visible to other packages.
    //     .executable(
    //         name: "Day15",
    //         targets: ["Day15"]),
    // ],
    targets: [
        .executableTarget(
            name: "Day15"),
        .testTarget(
            name: "Day15Tests",
            dependencies: ["Day15"]),
    ]
)
