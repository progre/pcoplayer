use objc2::{
    declare_class, msg_send_id, mutability::MainThreadOnly, rc::Retained, runtime::ProtocolObject,
    ClassType, DeclaredClass,
};
use objc2_app_kit::{NSWindow, NSWindowDelegate};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol, NSSize};

use super::{window_delegate_impl::window_will_resize_to_size, ResizerImpl};

pub struct Ivars {
    pub owner: *const ResizerImpl,
    pub orig_delegate: Retained<ProtocolObject<dyn NSWindowDelegate>>,
}

declare_class!(
    pub struct WindowDelegate;

    unsafe impl ClassType for WindowDelegate {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
        const NAME: &'static str = "WindowDelegate";
    }

    impl DeclaredClass for WindowDelegate {
        type Ivars = Ivars;
    }

    unsafe impl NSObjectProtocol for WindowDelegate {}

    // https://github.com/tauri-apps/tao/blob/tao-v0.30.2/src/platform_impl/macos/window_delegate.rs#L178
    #[allow(non_snake_case)]
    unsafe impl NSWindowDelegate for WindowDelegate {
        #[method(windowShouldClose:)]
        unsafe fn windowShouldClose(&self, sender: &NSWindow) -> bool {
            self.ivars().orig_delegate.windowShouldClose(sender)
        }

        // #[method_id(@__retain_semantics Other windowWillReturnFieldEditor:toObject:)]
        // unsafe fn windowWillReturnFieldEditor_toObject(
        //     &self,
        //     sender: &NSWindow,
        //     client: Option<&AnyObject>,
        // ) -> Option<Retained<AnyObject>>;

        #[method(windowWillResize:toSize:)]
        unsafe fn windowWillResize_toSize(&self, sender: &NSWindow, frame_size: NSSize) -> NSSize {
            // self.ivars().orig_delegate.windowWillResize_toSize(sender, frame_size);
            window_will_resize_to_size(self, sender, frame_size)
        }

        // #[method(windowWillUseStandardFrame:defaultFrame:)]
        // unsafe fn windowWillUseStandardFrame_defaultFrame(
        //     &self,
        //     window: &NSWindow,
        //     new_frame: NSRect,
        // ) -> NSRect {
        //     self.ivars()
        //         .orig_delegate
        //         .windowWillUseStandardFrame_defaultFrame(window, new_frame)
        // }

        // #[method(windowShouldZoom:toFrame:)]
        // unsafe fn windowShouldZoom_toFrame(&self, window: &NSWindow, new_frame: NSRect) -> bool {
        //     self.ivars().orig_delegate.windowShouldZoom_toFrame(window, new_frame)
        // }

        // #[method_id(@__retain_semantics Other windowWillReturnUndoManager:)]
        // unsafe fn windowWillReturnUndoManager(
        //     &self,
        //     window: &NSWindow,
        // ) -> Option<Retained<NSUndoManager>>;

        // #[method(window:willPositionSheet:usingRect:)]
        // unsafe fn window_willPositionSheet_usingRect(
        //     &self,
        //     window: &NSWindow,
        //     sheet: &NSWindow,
        //     rect: NSRect,
        // ) -> NSRect {
        //     self.ivars()
        //         .orig_delegate
        //         .window_willPositionSheet_usingRect(window, sheet, rect)
        // }

        // #[method(window:shouldPopUpDocumentPathMenu:)]
        // unsafe fn window_shouldPopUpDocumentPathMenu(
        //     &self,
        //     window: &NSWindow,
        //     menu: &NSMenu,
        // ) -> bool {
        //     self.ivars()
        //         .orig_delegate
        //         .window_shouldPopUpDocumentPathMenu(window, menu)
        // }

        // #[method(window:shouldDragDocumentWithEvent:from:withPasteboard:)]
        // unsafe fn window_shouldDragDocumentWithEvent_from_withPasteboard(
        //     &self,
        //     window: &NSWindow,
        //     event: &NSEvent,
        //     drag_image_location: NSPoint,
        //     pasteboard: &NSPasteboard,
        // ) -> bool {
        //     self.ivars()
        //         .orig_delegate
        //         .window_shouldDragDocumentWithEvent_from_withPasteboard(
        //             window,
        //             event,
        //             drag_image_location,
        //             pasteboard,
        //         )
        // }

        // #[method(window:willUseFullScreenContentSize:)]
        // unsafe fn window_willUseFullScreenContentSize(
        //     &self,
        //     window: &NSWindow,
        //     proposed_size: NSSize,
        // ) -> NSSize {
        //     self.ivars()
        //         .orig_delegate
        //         .window_willUseFullScreenContentSize(window, proposed_size)
        // }

        // #[method(window:willUseFullScreenPresentationOptions:)]
        // unsafe fn window_willUseFullScreenPresentationOptions(
        //     &self,
        //     window: &NSWindow,
        //     proposed_options: NSApplicationPresentationOptions,
        // ) -> NSApplicationPresentationOptions {
        //     self.ivars()
        //         .orig_delegate
        //         .window_willUseFullScreenPresentationOptions(window, proposed_options)
        // }

        // #[method_id(@__retain_semantics Other customWindowsToEnterFullScreenForWindow:)]
        // unsafe fn customWindowsToEnterFullScreenForWindow(
        //     &self,
        //     window: &NSWindow,
        // ) -> Option<Retained<NSArray<NSWindow>>>;

        // #[method(window:startCustomAnimationToEnterFullScreenWithDuration:)]
        // unsafe fn window_startCustomAnimationToEnterFullScreenWithDuration(
        //     &self,
        //     window: &NSWindow,
        //     duration: NSTimeInterval,
        // ) {
        //     self.ivars()
        //         .orig_delegate
        //         .window_startCustomAnimationToEnterFullScreenWithDuration(window, duration)
        // }

        // #[method(windowDidFailToEnterFullScreen:)]
        // unsafe fn windowDidFailToEnterFullScreen(&self, window: &NSWindow) {
        //     self.ivars()
        //         .orig_delegate
        //         .windowDidFailToEnterFullScreen(window)
        // }

        // #[method_id(@__retain_semantics Other customWindowsToExitFullScreenForWindow:)]
        // unsafe fn customWindowsToExitFullScreenForWindow(
        //     &self,
        //     window: &NSWindow,
        // ) -> Option<Retained<NSArray<NSWindow>>>;

        // #[method(window:startCustomAnimationToExitFullScreenWithDuration:)]
        // unsafe fn window_startCustomAnimationToExitFullScreenWithDuration(
        //     &self,
        //     window: &NSWindow,
        //     duration: NSTimeInterval,
        // ) {
        //     self.ivars()
        //         .orig_delegate
        //         .window_startCustomAnimationToExitFullScreenWithDuration(window, duration)
        // }

        // #[method_id(@__retain_semantics Other customWindowsToEnterFullScreenForWindow:onScreen:)]
        // unsafe fn customWindowsToEnterFullScreenForWindow_onScreen(
        //     &self,
        //     window: &NSWindow,
        //     screen: &NSScreen,
        // ) -> Option<Retained<NSArray<NSWindow>>>;

        // #[method(window:startCustomAnimationToEnterFullScreenOnScreen:withDuration:)]
        // unsafe fn window_startCustomAnimationToEnterFullScreenOnScreen_withDuration(
        //     &self,
        //     window: &NSWindow,
        //     screen: &NSScreen,
        //     duration: NSTimeInterval,
        // ) {
        //     self.ivars()
        //         .orig_delegate
        //         .window_startCustomAnimationToEnterFullScreenOnScreen_withDuration(
        //             window,
        //             screen,
        //             duration,
        //         )
        // }

        // #[method(windowDidFailToExitFullScreen:)]
        // unsafe fn windowDidFailToExitFullScreen(&self, window: &NSWindow) {
        //     self.ivars()
        //         .orig_delegate
        //         .windowDidFailToExitFullScreen(window)
        // }

        // #[method(window:willResizeForVersionBrowserWithMaxPreferredSize:maxAllowedSize:)]
        // unsafe fn window_willResizeForVersionBrowserWithMaxPreferredSize_maxAllowedSize(
        //     &self,
        //     window: &NSWindow,
        //     max_preferred_frame_size: NSSize,
        //     max_allowed_frame_size: NSSize,
        // ) -> NSSize {
        //     self.ivars()
        //         .orig_delegate
        //         .window_willResizeForVersionBrowserWithMaxPreferredSize_maxAllowedSize(
        //             window,
        //             max_preferred_frame_size,
        //             max_allowed_frame_size,
        //         )
        // }

        // #[method(window:willEncodeRestorableState:)]
        // unsafe fn window_willEncodeRestorableState(&self, window: &NSWindow, state: &NSCoder) {
        //     self.ivars()
        //         .orig_delegate
        //         .window_willEncodeRestorableState(window, state)
        // }

        // #[method(window:didDecodeRestorableState:)]
        // unsafe fn window_didDecodeRestorableState(&self, window: &NSWindow, state: &NSCoder) {
        //     self.ivars()
        //         .orig_delegate
        //         .window_didDecodeRestorableState(window, state)
        // }

        // #[method_id(@__retain_semantics Other previewRepresentableActivityItemsForWindow:)]
        // unsafe fn previewRepresentableActivityItemsForWindow(
        //     &self,
        //     window: &NSWindow,
        // ) -> Option<Retained<NSArray<ProtocolObject<dyn NSPreviewRepresentableActivityItem>>>>;

        #[method(windowDidResize:)]
        unsafe fn windowDidResize(&self, notification: &NSNotification) {
            self.ivars().orig_delegate.windowDidResize(notification)
        }

        // #[method(windowDidExpose:)]
        // unsafe fn windowDidExpose(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidExpose(notification)
        // }

        // #[method(windowWillMove:)]
        // unsafe fn windowWillMove(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowWillMove(notification)
        // }

        #[method(windowDidMove:)]
        unsafe fn windowDidMove(&self, notification: &NSNotification) {
            self.ivars().orig_delegate.windowDidMove(notification);
        }

        #[method(windowDidBecomeKey:)]
        unsafe fn windowDidBecomeKey(&self, notification: &NSNotification) {
            self.ivars().orig_delegate.windowDidBecomeKey(notification)
        }

        #[method(windowDidResignKey:)]
        unsafe fn windowDidResignKey(&self, notification: &NSNotification) {
            self.ivars().orig_delegate.windowDidResignKey(notification)
        }

        // #[method(windowDidBecomeMain:)]
        // unsafe fn windowDidBecomeMain(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidBecomeMain(notification)
        // }

        // #[method(windowDidResignMain:)]
        // unsafe fn windowDidResignMain(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidResignMain(notification)
        // }

        #[method(windowWillClose:)]
        unsafe fn windowWillClose(&self, notification: &NSNotification) {
            self.ivars().orig_delegate.windowWillClose(notification)
        }

        // #[method(windowWillMiniaturize:)]
        // unsafe fn windowWillMiniaturize(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowWillMiniaturize(notification)
        // }

        // #[method(windowDidMiniaturize:)]
        // unsafe fn windowDidMiniaturize(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidMiniaturize(notification)
        // }

        // #[method(windowDidDeminiaturize:)]
        // unsafe fn windowDidDeminiaturize(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidDeminiaturize(notification)
        // }

        // #[method(windowDidUpdate:)]
        // unsafe fn windowDidUpdate(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidUpdate(notification)
        // }

        // #[method(windowDidChangeScreen:)]
        // unsafe fn windowDidChangeScreen(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidChangeScreen(notification)
        // }

        // #[method(windowDidChangeScreenProfile:)]
        // unsafe fn windowDidChangeScreenProfile(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidChangeScreenProfile(notification)
        // }

        #[method(windowDidChangeBackingProperties:)]
        unsafe fn windowDidChangeBackingProperties(&self, notification: &NSNotification) {
            self.ivars().orig_delegate.windowDidChangeBackingProperties(notification)
        }

        // #[method(windowWillBeginSheet:)]
        // unsafe fn windowWillBeginSheet(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowWillBeginSheet(notification)
        // }

        // #[method(windowDidEndSheet:)]
        // unsafe fn windowDidEndSheet(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidEndSheet(notification)
        // }

        // #[method(windowWillStartLiveResize:)]
        // unsafe fn windowWillStartLiveResize(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowWillStartLiveResize(notification)
        // }

        // #[method(windowDidEndLiveResize:)]
        // unsafe fn windowDidEndLiveResize(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidEndLiveResize(notification)
        // }

        // #[method(windowWillEnterFullScreen:)]
        // unsafe fn windowWillEnterFullScreen(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowWillEnterFullScreen(notification)
        // }

        // #[method(windowDidEnterFullScreen:)]
        // unsafe fn windowDidEnterFullScreen(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidEnterFullScreen(notification)
        // }

        // #[method(windowWillExitFullScreen:)]
        // unsafe fn windowWillExitFullScreen(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowWillExitFullScreen(notification)
        // }

        // #[method(windowDidExitFullScreen:)]
        // unsafe fn windowDidExitFullScreen(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidExitFullScreen(notification)
        // }

        // #[method(windowWillEnterVersionBrowser:)]
        // unsafe fn windowWillEnterVersionBrowser(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowWillEnterVersionBrowser(notification)
        // }

        // #[method(windowDidEnterVersionBrowser:)]
        // unsafe fn windowDidEnterVersionBrowser(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidEnterVersionBrowser(notification)
        // }

        // #[method(windowWillExitVersionBrowser:)]
        // unsafe fn windowWillExitVersionBrowser(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowWillExitVersionBrowser(notification)
        // }

        // #[method(windowDidExitVersionBrowser:)]
        // unsafe fn windowDidExitVersionBrowser(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidExitVersionBrowser(notification)
        // }

        // #[method(windowDidChangeOcclusionState:)]
        // unsafe fn windowDidChangeOcclusionState(&self, notification: &NSNotification) {
        //     self.ivars().orig_delegate.windowDidChangeOcclusionState(notification)
        // }
    }
);

impl WindowDelegate {
    pub fn new(
        mtm: MainThreadMarker,
        owner: *const ResizerImpl,
        orig_delegate: Retained<ProtocolObject<dyn NSWindowDelegate>>,
    ) -> Retained<Self> {
        let zelf = mtm.alloc();
        let zelf = zelf.set_ivars(Ivars {
            owner,
            orig_delegate,
        });
        unsafe { msg_send_id![super(zelf), init] }
    }
}
