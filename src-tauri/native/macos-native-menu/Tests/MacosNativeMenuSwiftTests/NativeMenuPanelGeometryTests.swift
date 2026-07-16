import AppKit
import XCTest
@testable import MacosNativeMenuSwift

final class NativeMenuPanelGeometryTests: XCTestCase {
    private let panelSize = NSSize(width: 310, height: 400)
    private let visible = NSRect(x: 0, y: 0, width: 1440, height: 900)

    func testCentersHorizontallyUnderButton() {
        let button = NSRect(x: 700, y: 860, width: 28, height: 24)
        let origin = NativeMenuPanelGeometry.panelOrigin(
            buttonFrameOnScreen: button,
            panelSize: self.panelSize,
            visibleFrame: self.visible
        )
        let expectedX = button.midX - self.panelSize.width / 2
        XCTAssertEqual(origin.x, expectedX, accuracy: 0.001)
        XCTAssertEqual(
            origin.y,
            button.minY - self.panelSize.height - NativeMenuPanelGeometry.gap,
            accuracy: 0.001
        )
    }

    func testClampsToLeftEdgeOfVisibleFrame() {
        let button = NSRect(x: 4, y: 860, width: 28, height: 24)
        let origin = NativeMenuPanelGeometry.panelOrigin(
            buttonFrameOnScreen: button,
            panelSize: self.panelSize,
            visibleFrame: self.visible
        )
        XCTAssertEqual(origin.x, self.visible.minX + NativeMenuPanelGeometry.screenInset, accuracy: 0.001)
    }

    func testClampsToRightEdgeOfVisibleFrame() {
        let button = NSRect(x: 1420, y: 860, width: 28, height: 24)
        let origin = NativeMenuPanelGeometry.panelOrigin(
            buttonFrameOnScreen: button,
            panelSize: self.panelSize,
            visibleFrame: self.visible
        )
        let maxX = self.visible.maxX - self.panelSize.width - NativeMenuPanelGeometry.screenInset
        XCTAssertEqual(origin.x, maxX, accuracy: 0.001)
    }

    func testFlipsAboveWhenNotEnoughRoomBelow() {
        // Menu-bar-like button near bottom of a short visible frame.
        let shortVisible = NSRect(x: 0, y: 0, width: 1440, height: 200)
        let button = NSRect(x: 700, y: 50, width: 28, height: 24)
        let origin = NativeMenuPanelGeometry.panelOrigin(
            buttonFrameOnScreen: button,
            panelSize: self.panelSize,
            visibleFrame: shortVisible
        )
        XCTAssertEqual(
            origin.y,
            button.maxY + NativeMenuPanelGeometry.gap,
            accuracy: 0.001
        )
    }
}
