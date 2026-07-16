// swift-tools-version: 6.2
import PackageDescription

let package = Package(
    name: "MacosNativeMenuSwift",
    platforms: [
        // Tray Liquid Glass is macOS 26+ only (web/React UI stays web).
        .macOS(.v26),
    ],
    products: [
        .library(
            name: "MacosNativeMenuSwift",
            type: .static,
            targets: ["MacosNativeMenuSwift"]
        ),
    ],
    targets: [
        .target(
            name: "MacosNativeMenuSwift",
            exclude: [
                "Resources",
            ]
        ),
        // RustActionStub.swift supplies macos_native_menu_dispatch_action for test linking.
        .testTarget(
            name: "MacosNativeMenuSwiftTests",
            dependencies: ["MacosNativeMenuSwift"]
        ),
    ]
)
