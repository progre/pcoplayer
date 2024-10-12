use anyhow::Result;
use tauri::{Manager, Window};
use url::Url;

use crate::bbs;

use super::{
    resolve_url::{self, UrlType},
    state::{state, StreamInfo},
    window::{set_video_size, update_height, window_frame_size},
};

#[tauri::command(rename_all = "camelCase")]
pub async fn initialize(
    window: Window,
    inner_width: u32,
    inner_height: u32,
    video_client_width: u32,
    video_client_height: u32,
) -> Result<StreamInfo, ()> {
    let window_frame_size = window_frame_size(&window, inner_width, inner_height);
    let state = state(window.app_handle());
    let mut state = state.lock().unwrap();
    let state = state.window_state_mut(window.label());
    let Some(window_frame_size) = window_frame_size else {
        return Ok(state.stream_info.clone());
    };
    if state.resizer.is_initialized_window_frame_size() {
        return Ok(state.stream_info.clone());
    }

    state.resizer.init_window_frame_size(window_frame_size);
    set_video_size(state, video_client_width, video_client_height);
    update_height(&window, state);

    Ok(state.stream_info.clone())
}

#[derive(serde::Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ResolveUrlResult {
    #[serde(rename_all = "camelCase")]
    Bbs {
        thread_url: String,
        charset: String,
        thread_name: String,
    },
    #[serde(rename_all = "camelCase")]
    Stream { stream_url: String },
    #[serde(rename_all = "camelCase")]
    Unknown { error: Option<String> },
}

#[tauri::command]
pub async fn resolve_url(url: String) -> ResolveUrlResult {
    let url_type = resolve_url::resolve_url(&url).await;
    match url_type {
        Err(e) => ResolveUrlResult::Unknown {
            error: Some(e.to_string()),
        },
        Ok(UrlType::Bbs {
            thread_url,
            charset,
            thread_name,
        }) => ResolveUrlResult::Bbs {
            thread_url: thread_url.to_string(),
            charset,
            thread_name,
        },
        Ok(UrlType::Stream { stream_url }) => ResolveUrlResult::Stream { stream_url },
        Ok(UrlType::Unknown) => ResolveUrlResult::Unknown { error: None },
    }
}

#[tauri::command]
pub async fn post(
    url: String,
    charset: String,
    name: String,
    email: String,
    msg: String,
) -> Result<(), String> {
    let url = Url::parse(&url).map_err(|x| x.to_string())?;
    let bbs = bbs::new(&url).await.map_err(|x| x.to_string())?;
    bbs.post(&charset, &name, &email, &msg)
        .await
        .map_err(|x| x.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn resize_video(window: Window, width: u32, height: u32) {
    let state = state(window.app_handle());
    let mut state = state.lock().unwrap();
    let state = state.window_state_mut(window.label());
    set_video_size(state, width, height);
}

#[tauri::command(rename_all = "camelCase")]
pub fn resize_interface(window: Window, interface_height: u32) {
    let state = state(window.app_handle());
    let mut state = state.lock().unwrap();
    let state = state.window_state_mut(window.label());
    state.resizer.set_interface_height(interface_height);
    update_height(&window, state);
}
