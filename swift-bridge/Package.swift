// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "AudioUnitBridge",
    platforms: [.macOS(.v13)],
    products: [
        .library(
            name: "AudioUnitBridge",
            type: .static,
            targets: ["AudioUnitBridge"]
        ),
    ],
    targets: [
        .target(
            name: "AudioUnitBridge",
            path: "Sources/AudioUnitBridge",
            publicHeadersPath: "include"
        ),
    ]
)
