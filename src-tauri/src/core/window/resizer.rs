#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::Resizer;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::Resizer;

use tauri::PhysicalSize;

struct ResizerParams {
    window_frame_size: PhysicalSize<u32>,
    video_size: PhysicalSize<u32>,
    interface_height: u32,
}

impl ResizerParams {
    pub fn new() -> Self {
        Self {
            window_frame_size: PhysicalSize::new(0, 0),
            video_size: PhysicalSize::new(16, 9),
            interface_height: 0,
        }
    }

    pub fn is_initialized_window_frame_size(&self) -> bool {
        self.window_frame_size != PhysicalSize::new(0, 0)
    }

    pub fn video_view_width(&self, window_width: f64) -> f64 {
        window_width - self.window_frame_size.width as f64
    }

    pub fn video_view_height(&self, window_height: f64) -> f64 {
        window_height - self.window_frame_size.height as f64 - self.interface_height as f64
    }

    pub fn adjusted_window_height(&self, window_width: f64) -> f64 {
        let video_aspect_ratio = {
            let orig_video_size = self.video_size;
            orig_video_size.width as f64 / orig_video_size.height as f64
        };
        let new_video_view_height = self.video_view_width(window_width) / video_aspect_ratio;
        self.window_frame_size.height as f64 + new_video_view_height + self.interface_height as f64
    }

    pub fn adjusted_window_width(&self, window_height: f64) -> f64 {
        let video_aspect_ratio = {
            let orig_video_size = self.video_size;
            orig_video_size.width as f64 / orig_video_size.height as f64
        };
        let new_video_view_width = self.video_view_height(window_height) * video_aspect_ratio;
        new_video_view_width + self.window_frame_size.width as f64
    }
}
