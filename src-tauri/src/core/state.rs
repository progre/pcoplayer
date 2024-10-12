use std::{collections::HashMap, sync::Mutex};

use tauri::{App, AppHandle, Manager, Runtime, State, WebviewWindow};

use crate::core::window::resizer::Resizer;

#[derive(Clone, Debug, clap::Args, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamInfo {
    pub url: Option<String>,
    pub channel_name: Option<String>,
    pub contact_url: Option<String>,
}

pub struct WindowState {
    pub resizer: Resizer,
    pub stream_info: StreamInfo,
}

pub struct AppState {
    window_states: HashMap<String, WindowState>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            window_states: HashMap::new(),
        }
    }

    pub fn window_state_mut(&mut self, label: &str) -> &mut WindowState {
        self.window_states.get_mut(label).unwrap()
    }

    pub fn init_window(&mut self, window: &WebviewWindow, stream_info: StreamInfo) {
        self.window_states.insert(
            window.label().to_owned(),
            WindowState {
                resizer: Resizer::new(window),
                stream_info,
            },
        );
    }
}

pub fn init_state(app: &App, stream_info: StreamInfo) {
    let mut state = AppState::new();
    let window = app.get_webview_window("main").unwrap();
    state.init_window(&window, stream_info);
    app.manage(Mutex::new(state));
}

pub fn state(app_handle: &AppHandle<impl Runtime>) -> State<'_, Mutex<AppState>> {
    app_handle.state::<Mutex<AppState>>()
}
