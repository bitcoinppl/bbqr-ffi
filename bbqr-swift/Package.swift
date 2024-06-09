// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
import PackageDescription

let package = Package(
    name: "BbqrSwift",
    platforms: [
        .macOS(.v12),
        .iOS(.v15)
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "Bbqr",
            targets: ["bbqrFFI"]),
    ],
    dependencies: [],
    targets: [
        .binaryTarget(name: "bbqrFFI", path: "./bbqrFFI.xcframework"),
        .target(
            name: "Bbqr",
            dependencies: ["bbqrFFI"]
        ),

    ]
)
