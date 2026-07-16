import Foundation

/// Test-only definition of the Rust tray bridge symbol so the static library links.
@_cdecl("macos_native_menu_dispatch_action")
public func macos_native_menu_dispatch_action_stub(
    _ action: UnsafePointer<CChar>?,
    _ platformId: UnsafePointer<CChar>?,
    _ accountId: UnsafePointer<CChar>?
) {
    _ = action
    _ = platformId
    _ = accountId
}
