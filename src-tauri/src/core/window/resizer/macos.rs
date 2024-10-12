mod window_delegate;
mod window_delegate_impl;

use std::pin::Pin;

use log::trace;
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_app_kit::{NSWindow, NSWindowDelegate};
use objc2_foundation::MainThreadMarker;
use tauri::{PhysicalSize, WebviewWindow};
use window_delegate::WindowDelegate;

use super::ResizerParams;

struct ResizerImpl {
    params: ResizerParams,
    // keep the delegate alive
    orig_delegate: Option<Retained<ProtocolObject<dyn NSWindowDelegate>>>,
    delegate: Option<Retained<WindowDelegate>>,
}

pub struct Resizer(Pin<Box<ResizerImpl>>);

impl Resizer {
    pub fn new(webview_window: &WebviewWindow) -> Self {
        let mut pin = Box::pin(ResizerImpl {
            params: ResizerParams::new(),
            orig_delegate: None,
            delegate: None,
        });

        let window = webview_window.ns_window().unwrap() as *const NSWindow;
        let window = unsafe { window.as_ref() }.unwrap();
        let delegate = WindowDelegate::new(
            MainThreadMarker::new().unwrap(),
            pin.as_ref().get_ref(),
            unsafe { window.delegate().unwrap() },
        );
        pin.orig_delegate = unsafe { window.delegate() };
        window.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));

        pin.delegate = Some(delegate);

        Self(pin)
    }

    pub fn is_initialized_window_frame_size(&self) -> bool {
        self.0.params.is_initialized_window_frame_size()
    }

    pub fn init_window_frame_size(&mut self, window_frame_size: PhysicalSize<u32>) {
        self.0.params.window_frame_size = window_frame_size;
    }

    pub fn set_video_size(&mut self, size: PhysicalSize<u32>) {
        self.0.params.video_size = size;
    }

    pub fn set_interface_height(&mut self, height: u32) {
        self.0.params.interface_height = height;
    }

    pub fn adjusted_window_height(&self, width: f64) -> f64 {
        self.0.params.adjusted_window_height(width)
    }
}

unsafe impl Send for Resizer {}
unsafe impl Sync for Resizer {}
