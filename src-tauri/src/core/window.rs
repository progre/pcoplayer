pub mod resizer;

use tauri::{PhysicalSize, Window};

use super::state::WindowState;

pub fn window_frame_size(
    window: &Window,
    inner_width: u32,
    inner_height: u32,
) -> Option<PhysicalSize<u32>> {
    let outer_size = window.outer_size().unwrap();
    let inner_size = PhysicalSize {
        width: inner_width,
        height: inner_height,
    };
    Some(PhysicalSize {
        width: outer_size.width.checked_sub(inner_size.width)?,
        height: outer_size.height.checked_sub(inner_size.height)?,
    })
}

pub fn set_video_size(state: &mut WindowState, width: u32, height: u32) {
    state.resizer.set_video_size(PhysicalSize { width, height });
}

pub fn update_height(window: &Window, state: &WindowState) {
    if !state.resizer.is_initialized_window_frame_size() {
        return;
    }
    let mut outer_size = window.outer_size().unwrap();
    outer_size.height = state
        .resizer
        .adjusted_window_height(outer_size.width as f64) as u32;
    window.set_size(outer_size).unwrap();
}
