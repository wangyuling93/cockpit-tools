import AppKit
import Combine
import SwiftUI

/// Pure geometry for anchoring the tray glass panel under the status item.
enum NativeMenuPanelGeometry {
    static let gap: CGFloat = 6
    static let screenInset: CGFloat = 8

    static func panelOrigin(
        buttonFrameOnScreen: NSRect,
        panelSize: NSSize,
        visibleFrame: NSRect,
        gap: CGFloat = gap
    ) -> NSPoint {
        var origin = NSPoint(
            x: buttonFrameOnScreen.midX - panelSize.width / 2,
            y: buttonFrameOnScreen.minY - panelSize.height - gap
        )
        origin.x = min(
            max(origin.x, visibleFrame.minX + screenInset),
            visibleFrame.maxX - panelSize.width - screenInset
        )
        if origin.y < visibleFrame.minY + screenInset {
            // Flip above the button if there is not enough room below (rare for menu bar).
            origin.y = buttonFrameOnScreen.maxY + gap
        }
        return origin
    }
}

/// Tray menu: borderless transparent `NSPanel` + AppKit `NSGlassEffectView` shell.
/// (Not NSPopover — solid popover chrome blocks Liquid Glass desktop sampling.)
/// Requires macOS 26+ for glass APIs. Main window content remains React/WebView.
@MainActor
final class NativeMenuPanelController: ObservableObject {
    static let shared = NativeMenuPanelController()

    @Published private(set) var snapshot: NativeMenuSnapshot?
    @Published private(set) var selectedPlatformId: String = ""
    @Published private(set) var viewedAccountIds: [String: String] = [:]
    @Published private(set) var refreshingPlatformId: String?
    @Published private(set) var refreshingAccountId: String?

    private weak var statusItem: NSStatusItem?
    private var panel: NSPanel?
    private var hostingController: NSHostingController<NativeMenuRootView>?
    private var localEventMonitor: Any?
    private var globalEventMonitor: Any?
    private var refreshStartedAt: Date?
    private var clearRefreshTask: Task<Void, Never>?

    var isPanelVisible: Bool {
        self.panel?.isVisible == true
    }

    func toggle(snapshotJSON: String, statusItemPointer: UnsafeMutableRawPointer) {
        guard let snapshot = self.decodeSnapshot(from: snapshotJSON) else { return }

        let statusItem = Unmanaged<NSStatusItem>.fromOpaque(statusItemPointer).takeUnretainedValue()
        self.statusItem = statusItem
        self.apply(snapshot: snapshot)
        self.ensurePanel()
        self.layoutGlassContent()

        if self.isPanelVisible {
            self.closePanel()
        } else {
            self.presentPanel()
        }
    }

    func selectPlatform(id: String) {
        guard let snapshot, snapshot.platforms.contains(where: { $0.id == id }) else {
            return
        }

        self.selectedPlatformId = id
        if self.viewedAccountIds[id] == nil,
           let platform = snapshot.platforms.first(where: { $0.id == id }),
           let initialAccountId = platform.currentOrFirstAccountId
        {
            self.viewedAccountIds[id] = initialAccountId
        }
        self.refreshPanelLayout()
    }

    func moveViewedAccount(delta: Int) {
        guard let platform = self.selectedPlatform, !platform.cards.isEmpty else {
            return
        }

        let currentId = self.viewedAccountIds[platform.id] ?? platform.currentOrFirstAccountId
        let currentIndex = platform.cards.firstIndex(where: { $0.id == currentId }) ?? 0
        let total = platform.cards.count
        let nextIndex = (currentIndex + delta + total) % total
        self.viewedAccountIds[platform.id] = platform.cards[nextIndex].id
        self.refreshPanelLayout()
    }

    func jumpToRecommendedAccount() {
        guard let platform = self.selectedPlatform,
              let recommendedId = platform.recommended_account_id,
              platform.cards.contains(where: { $0.id == recommendedId })
        else {
            return
        }
        self.viewedAccountIds[platform.id] = recommendedId
        self.refreshPanelLayout()
    }

    func jumpBackToCurrentAccount() {
        guard let platform = self.selectedPlatform,
              let currentId = platform.current_account_id,
              platform.cards.contains(where: { $0.id == currentId })
        else {
            return
        }
        self.viewedAccountIds[platform.id] = currentId
        self.refreshPanelLayout()
    }

    func dispatch(action: NativeRustAction) {
        switch action {
        case .refresh:
            guard let platform = self.selectedPlatform else { return }
            let accountId = self.viewedCard?.id
            guard !self.isRefreshing(platformId: platform.id, accountId: accountId) else { return }
            self.beginRefresh(platformId: platform.id, accountId: accountId)
            dispatchRustMenuAction(
                action: "refresh",
                platformId: platform.id,
                accountId: accountId
            )
        case .switchAccount:
            guard let platform = self.selectedPlatform, let viewedCard = self.viewedCard else { return }
            self.closePanel()
            dispatchRustMenuAction(action: "switch", platformId: platform.id, accountId: viewedCard.id)
        case .openDetails:
            guard let platform = self.selectedPlatform else { return }
            self.closePanel()
            dispatchRustMenuAction(action: "open_details", platformId: platform.id)
        case .viewAllAccounts:
            guard let platform = self.selectedPlatform else { return }
            self.closePanel()
            dispatchRustMenuAction(action: "view_all_accounts", platformId: platform.id)
        case .openCockpitTools:
            self.closePanel()
            dispatchRustMenuAction(action: "open_cockpit_tools")
        case .settings:
            self.closePanel()
            dispatchRustMenuAction(action: "settings")
        case .quit:
            self.closePanel()
            dispatchRustMenuAction(action: "quit")
        }
    }

    var selectedPlatform: NativeMenuPlatform? {
        guard let snapshot else {
            return nil
        }
        return snapshot.platforms.first(where: { $0.id == self.selectedPlatformId }) ?? snapshot.platforms.first
    }

    var viewedCard: NativeMenuAccountCard? {
        guard let platform = self.selectedPlatform else {
            return nil
        }
        let viewedId = self.viewedAccountIds[platform.id] ?? platform.currentOrFirstAccountId
        return platform.cards.first(where: { $0.id == viewedId }) ?? platform.cards.first
    }

    func isViewingCurrentAccount(for platform: NativeMenuPlatform) -> Bool {
        let viewedId = self.viewedAccountIds[platform.id] ?? platform.currentOrFirstAccountId
        return viewedId == platform.current_account_id
    }

    func shouldShowRecommendedAction(for platform: NativeMenuPlatform) -> Bool {
        self.isViewingCurrentAccount(for: platform)
            && platform.recommended_account_id != nil
            && platform.recommended_account_id != platform.current_account_id
    }

    func shouldShowBackAction(for platform: NativeMenuPlatform) -> Bool {
        !self.isViewingCurrentAccount(for: platform) && platform.current_account_id != nil
    }

    func shouldShowSwitchAction(for platform: NativeMenuPlatform) -> Bool {
        !self.isViewingCurrentAccount(for: platform) && self.viewedCard != nil
    }

    func isRefreshing(platformId: String, accountId: String?) -> Bool {
        self.refreshingPlatformId == platformId && self.refreshingAccountId == accountId
    }

    func update(snapshotJSON: String) {
        guard let snapshot = self.decodeSnapshot(from: snapshotJSON) else { return }
        self.apply(snapshot: snapshot)
        self.finishRefreshIfNeeded()
        self.refreshPanelLayout()
    }

    // MARK: - Snapshot

    private func decodeSnapshot(from json: String) -> NativeMenuSnapshot? {
        guard let data = json.data(using: .utf8) else { return nil }
        do {
            return try JSONDecoder().decode(NativeMenuSnapshot.self, from: data)
        } catch {
            return nil
        }
    }

    private func apply(snapshot: NativeMenuSnapshot) {
        self.snapshot = snapshot

        let validPlatformIds = Set(snapshot.platforms.map(\.id))
        if !validPlatformIds.contains(self.selectedPlatformId) {
            self.selectedPlatformId = snapshot.selected_platform_id
        }
        if self.selectedPlatformId.isEmpty {
            self.selectedPlatformId = snapshot.selected_platform_id
        }

        var nextViewedAccountIds = self.viewedAccountIds
        for platform in snapshot.platforms {
            let currentViewedId = nextViewedAccountIds[platform.id]
            if let currentViewedId,
               platform.cards.contains(where: { $0.id == currentViewedId })
            {
                continue
            }

            if let fallback = platform.currentOrFirstAccountId {
                nextViewedAccountIds[platform.id] = fallback
            } else {
                nextViewedAccountIds.removeValue(forKey: platform.id)
            }
        }
        self.viewedAccountIds = nextViewedAccountIds
    }

    // MARK: - Glass panel shell

    private func ensurePanel() {
        if self.panel != nil {
            return
        }

        // Host once; @ObservedObject drives content updates (no root remount on toggle).
        let hosting = TransparentHostingController(rootView: NativeMenuRootView(controller: self))
        hosting.sizingOptions = [.intrinsicContentSize]
        // Single layout owner: panel.setContentSize resizes glass; hosting fills glass via autoresizing.
        hosting.view.autoresizingMask = [.width, .height]

        let glass = NSGlassEffectView(frame: .zero)
        glass.style = .clear
        glass.cornerRadius = NativeMenuLayout.cornerRadius
        glass.tintColor = nil
        if #available(macOS 27.0, *) {
            // GitHub's macOS 26 runner has an SDK 26 toolchain, which cannot
            // type-check this macOS 27 beta property. Keep the runtime feature
            // without making the package require an SDK 27 compiler.
            let setter = NSSelectorFromString("setEffectIsInteractive:")
            if glass.responds(to: setter) {
                glass.setValue(true, forKey: "effectIsInteractive")
            }
        }
        glass.autoresizingMask = [.width, .height]
        glass.contentView = hosting.view

        let panel = NSPanel(
            contentRect: NSRect(x: 0, y: 0, width: NativeMenuLayout.width, height: NativeMenuLayout.minHeight),
            styleMask: [.borderless, .nonactivatingPanel],
            backing: .buffered,
            defer: false
        )
        panel.isOpaque = false
        panel.backgroundColor = .clear
        panel.hasShadow = true
        panel.level = .statusBar
        panel.collectionBehavior = [.canJoinAllSpaces, .fullScreenAuxiliary, .transient]
        panel.isMovableByWindowBackground = false
        panel.hidesOnDeactivate = false
        panel.becomesKeyOnlyIfNeeded = true
        panel.isReleasedWhenClosed = false
        panel.contentView = glass

        self.hostingController = hosting
        // glass is retained as panel.contentView; no separate shell field needed.
        self.panel = panel
    }

    /// One size owner: measure SwiftUI, then `setContentSize` only.
    /// AppKit resizes the glass content view; hosting fills glass via autoresizingMask.
    private func layoutGlassContent() {
        guard let hostingController, let panel else {
            return
        }

        hostingController.view.layoutSubtreeIfNeeded()
        let fitting = hostingController.sizeThatFits(in: NSSize(
            width: NativeMenuLayout.width,
            height: CGFloat.greatestFiniteMagnitude
        ))
        let width = max(NativeMenuLayout.width, ceil(fitting.width))
        let height = max(NativeMenuLayout.minHeight, ceil(fitting.height))
        panel.setContentSize(NSSize(width: width, height: height))
    }

    private func presentPanel() {
        guard let panel, let button = self.statusItem?.button else {
            return
        }

        self.layoutGlassContent()
        self.repositionPanel(relativeTo: button)
        button.highlight(true)
        panel.orderFrontRegardless()
        self.installDismissMonitors()
    }

    private func repositionPanel(relativeTo button: NSButton) {
        guard let panel, let buttonWindow = button.window else {
            return
        }
        let buttonFrameInWindow = button.convert(button.bounds, to: nil)
        let buttonFrameOnScreen = buttonWindow.convertToScreen(buttonFrameInWindow)
        let visible = (buttonWindow.screen ?? NSScreen.main)?.visibleFrame
            ?? NSRect(origin: .zero, size: panel.frame.size)
        let origin = NativeMenuPanelGeometry.panelOrigin(
            buttonFrameOnScreen: buttonFrameOnScreen,
            panelSize: panel.frame.size,
            visibleFrame: visible
        )
        panel.setFrameOrigin(origin)
    }

    private func closePanel() {
        self.removeDismissMonitors()
        self.panel?.orderOut(nil)
        self.clearStatusItemHighlight()
    }

    private func clearStatusItemHighlight() {
        guard let button = self.statusItem?.button else {
            return
        }
        button.highlight(false)
        button.needsDisplay = true
        button.displayIfNeeded()
    }

    /// Already `@MainActor` — layout synchronously (no main.async hop).
    private func refreshPanelLayout() {
        self.layoutGlassContent()
        if self.isPanelVisible, let button = self.statusItem?.button {
            self.repositionPanel(relativeTo: button)
        }
    }

    // MARK: - Dismiss (single policy for local + global)

    /// Shared predicate so local/global monitors cannot disagree.
    /// Status-item clicks are owned exclusively by Rust `toggle` — never close here.
    private func shouldDismiss(for event: NSEvent) -> Bool {
        if event.type == .keyDown {
            return event.keyCode == 53 // Esc
        }

        guard event.type == .leftMouseDown
            || event.type == .rightMouseDown
            || event.type == .otherMouseDown
        else {
            return false
        }

        // Clicks inside the glass panel stay open.
        if let panel = self.panel, event.window === panel {
            return false
        }

        // Status item button: let Rust toggle handle open/close (avoid close-then-reopen race).
        if self.isEventInStatusItemButton(event) {
            return false
        }

        return true
    }

    private func isEventInStatusItemButton(_ event: NSEvent) -> Bool {
        guard let button = self.statusItem?.button,
              let buttonWindow = button.window
        else {
            return false
        }

        // Same window as the status item: convert into button coords.
        if event.window === buttonWindow {
            let locationInButton = button.convert(event.locationInWindow, from: nil)
            return button.bounds.contains(locationInButton)
        }

        // Global monitor events often have nil window — use screen frames.
        let buttonFrameInWindow = button.convert(button.bounds, to: nil)
        let buttonFrameOnScreen = buttonWindow.convertToScreen(buttonFrameInWindow)
        return buttonFrameOnScreen.contains(NSEvent.mouseLocation)
    }

    private func installDismissMonitors() {
        self.removeDismissMonitors()

        self.localEventMonitor = NSEvent.addLocalMonitorForEvents(
            matching: [.leftMouseDown, .rightMouseDown, .otherMouseDown, .keyDown]
        ) { [weak self] event in
            guard let self else { return event }
            if self.shouldDismiss(for: event) {
                self.closePanel()
                // Swallow Esc; let mouse events continue so the underlying click still lands.
                return event.type == .keyDown ? nil : event
            }
            return event
        }

        self.globalEventMonitor = NSEvent.addGlobalMonitorForEvents(
            matching: [.leftMouseDown, .rightMouseDown]
        ) { [weak self] event in
            guard let self else { return }
            if self.shouldDismiss(for: event) {
                self.closePanel()
            }
        }
    }

    private func removeDismissMonitors() {
        if let localEventMonitor {
            NSEvent.removeMonitor(localEventMonitor)
            self.localEventMonitor = nil
        }
        if let globalEventMonitor {
            NSEvent.removeMonitor(globalEventMonitor)
            self.globalEventMonitor = nil
        }
    }

    // MARK: - Refresh spinner

    private func beginRefresh(platformId: String, accountId: String?) {
        self.clearRefreshTask?.cancel()
        self.refreshStartedAt = Date()
        self.refreshingPlatformId = platformId
        self.refreshingAccountId = accountId
        self.refreshPanelLayout()
    }

    private func finishRefreshIfNeeded() {
        guard let refreshingPlatformId else {
            return
        }
        let refreshingAccountId = self.refreshingAccountId
        let elapsed = Date().timeIntervalSince(self.refreshStartedAt ?? .distantPast)
        let remainingDelay = max(0, 0.45 - elapsed)

        self.clearRefreshTask?.cancel()
        self.clearRefreshTask = Task { [weak self] in
            if remainingDelay > 0 {
                try? await Task.sleep(nanoseconds: UInt64(remainingDelay * 1_000_000_000))
            }
            guard !Task.isCancelled else {
                return
            }
            await MainActor.run {
                guard let self,
                      self.refreshingPlatformId == refreshingPlatformId,
                      self.refreshingAccountId == refreshingAccountId
                else {
                    return
                }
                self.refreshingPlatformId = nil
                self.refreshingAccountId = nil
                self.refreshStartedAt = nil
                self.refreshPanelLayout()
            }
        }
    }
}

/// Transparent hosting so AppKit `NSGlassEffectView` can show desktop through the shell.
private final class TransparentHostingController<Content: View>: NSHostingController<Content> {
    override func viewDidLoad() {
        super.viewDidLoad()
        self.view.wantsLayer = true
        self.view.layer?.backgroundColor = NSColor.clear.cgColor
    }
}

enum NativeRustAction {
    case refresh
    case switchAccount
    case openDetails
    case viewAllAccounts
    case openCockpitTools
    case settings
    case quit
}
