mod bbs;
mod core;

use core::commands;

use clap::Parser;
use tauri::Manager;

use crate::core::{
    platform,
    state::{init_state, StreamInfo},
};

#[derive(Clone, Debug, clap::Parser, serde::Serialize)]
#[command(version, about, long_about = None)]
#[serde(rename_all = "camelCase")]
struct Args {
    #[clap(flatten)]
    stream_info: StreamInfo,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let args = Args::parse();

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::initialize,
            commands::resolve_url,
            commands::post,
            commands::resize_video,
            commands::resize_interface,
        ])
        .setup(move |app| {
            platform::setup(app.app_handle().to_owned());
            init_state(app, args.stream_info);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
