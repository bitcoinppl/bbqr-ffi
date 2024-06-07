// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
import PackageDescription

let package = Package(
    name: "bbqr-swift",
    platforms: [
        .macOS(.v12),
        .iOS(.v15)
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "Bbqr",
            targets: ["bbqrFFI", "Bbqr"]),
    ],
    dependencies: [],
        .binaryTarget(name: "bdkFFI", path: "./bdkFFI.xcframework"),
        .target(
            name: "Bbqr",
            dependencies: ["bbqrFFI"]
        ),
    ]
)
