use objc2::DeclaredClass;
use objc2_app_kit::NSWindow;
use objc2_foundation::NSSize;

use super::window_delegate::WindowDelegate;

pub fn window_will_resize_to_size(
    zelf: &WindowDelegate,
    sender: &NSWindow,
    mut frame_size: NSSize,
) -> NSSize {
    let owner = zelf.ivars().owner;
    let owner = unsafe { owner.as_ref() }.unwrap();

    // NOTE: The macOS API is tooooo complex!

    if frame_size.width < 320.0 {
        frame_size.width = 320.0;
    }
    let scale_factor = sender.screen().unwrap().backingScaleFactor();
    log::trace!(
        "frame_size.width: {}, scale_factor: {}",
        frame_size.width,
        scale_factor
    );
    let physical_width = frame_size.width * scale_factor;
    frame_size.height = owner.params.adjusted_window_height(physical_width) / scale_factor;
    log::trace!(
        "height: {}, frame_size.height: {}",
        owner.params.adjusted_window_height(physical_width),
        frame_size.height
    );
    frame_size
}
