use std::{mem::transmute, pin::Pin, ptr::null};

use log::debug;
use tauri::{PhysicalSize, WebviewWindow};
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{HANDLE, HWND, LPARAM, LRESULT, RECT, WPARAM},
        UI::WindowsAndMessaging::{
            GetPropW, GetWindowLongPtrW, SetPropW, SetWindowLongPtrW, GWLP_WNDPROC, MINMAXINFO,
            WMSZ_BOTTOM, WMSZ_BOTTOMLEFT, WMSZ_BOTTOMRIGHT, WMSZ_LEFT, WMSZ_RIGHT, WMSZ_TOP,
            WMSZ_TOPLEFT, WMSZ_TOPRIGHT, WM_GETMINMAXINFO, WM_SIZING,
        },
    },
};

use super::ResizerParams;

type WndProc = extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT;

fn owner(hwnd: HWND) -> &'static ResizerImpl {
    let key = HSTRING::from("PCOPLAYER_RESIZER_OWNER");
    let owner =
        unsafe { transmute::<HANDLE, *const ResizerImpl>(GetPropW(hwnd, &key)).as_ref() }.unwrap();
    owner
}

fn set_owner(hwnd: HWND, owner: *const ResizerImpl) {
    let key = HSTRING::from("PCOPLAYER_RESIZER_OWNER");
    unsafe { SetPropW(hwnd, &key, HANDLE(owner as _)) }.unwrap();
}

enum ResizeOverride {
    Top,
    Bottom,
    Left,
    Right,
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let resizer = owner(hwnd);
    if msg != WM_SIZING {
        return (resizer.orig_wnd_proc)(hwnd, msg, wparam, lparam);
    }
    let wmsz = wparam.0 as u32;
    let rect = lparam.0 as *mut RECT;
    let rect = unsafe { rect.as_mut() }.unwrap();

    let video_aspect_ratio = {
        let orig_video_size = resizer.params.video_size;
        orig_video_size.width as f64 / orig_video_size.height as f64
    };

    let window_width = rect.right - rect.left;
    let window_height = rect.bottom - rect.top;
    let video_view_width = resizer.params.video_view_width(window_width as f64);
    let video_view_height = resizer.params.video_view_height(window_height as f64);
    let video_view_aspect_ratio = video_view_width / video_view_height;

    let resize_override = match wmsz {
        WMSZ_TOP | WMSZ_BOTTOM => ResizeOverride::Right,
        WMSZ_LEFT | WMSZ_RIGHT => ResizeOverride::Bottom,
        WMSZ_TOPLEFT => {
            if video_aspect_ratio < video_view_aspect_ratio {
                ResizeOverride::Left
            } else {
                ResizeOverride::Top
            }
        }
        WMSZ_TOPRIGHT => {
            if video_aspect_ratio < video_view_aspect_ratio {
                ResizeOverride::Right
            } else {
                ResizeOverride::Top
            }
        }
        WMSZ_BOTTOMLEFT => {
            if video_aspect_ratio < video_view_aspect_ratio {
                ResizeOverride::Left
            } else {
                ResizeOverride::Bottom
            }
        }
        WMSZ_BOTTOMRIGHT => {
            if video_aspect_ratio < video_view_aspect_ratio {
                ResizeOverride::Right
            } else {
                ResizeOverride::Bottom
            }
        }
        _ => panic!("WMSZ: {}", wmsz),
    };
    match resize_override {
        ResizeOverride::Top => {
            rect.top =
                rect.bottom - resizer.params.adjusted_window_height(window_width as f64) as i32;
        }
        ResizeOverride::Left => {
            rect.left =
                rect.right - resizer.params.adjusted_window_width(window_height as f64) as i32;
        }
        ResizeOverride::Right => {
            rect.right =
                rect.left + resizer.params.adjusted_window_width(window_height as f64) as i32;
        }
        ResizeOverride::Bottom => {
            rect.bottom =
                rect.top + resizer.params.adjusted_window_height(window_width as f64) as i32;
        }
    }
    LRESULT(1)
}

struct ResizerImpl {
    params: ResizerParams,
    hwnd: HWND,
    orig_wnd_proc: WndProc,
}

pub struct Resizer(Pin<Box<ResizerImpl>>);

impl Resizer {
    pub fn new(webview_window: &WebviewWindow) -> Self {
        let hwnd = webview_window.hwnd().unwrap();
        let orig_wnd_proc =
            unsafe { transmute::<isize, WndProc>(GetWindowLongPtrW(hwnd, GWLP_WNDPROC)) };
        unsafe { SetWindowLongPtrW(hwnd, GWLP_WNDPROC, wndproc as usize as isize) };
        let pin = Box::pin(ResizerImpl {
            params: ResizerParams::new(),
            hwnd,
            orig_wnd_proc,
        });
        set_owner(hwnd, pin.as_ref().get_ref());
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

impl Drop for Resizer {
    fn drop(&mut self) {
        set_owner(self.0.hwnd, null());
    }
}

unsafe impl Send for Resizer {}
unsafe impl Sync for Resizer {}
