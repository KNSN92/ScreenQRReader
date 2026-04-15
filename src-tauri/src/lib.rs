use std::error::Error;

use anyhow::Result;
use log::info;
use tauri::App;

pub use crate::i18n::i18n_translations;
pub use crate::qr_maker::{generate_qrcode, validate_qrcode};

mod config;
mod hotkey;
mod i18n;
mod misc;
mod platform;
mod qr_maker;
mod qr_reader;
mod screenshot;
mod tray;
mod updater;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_http::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            i18n_translations,
            generate_qrcode,
            validate_qrcode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    tauri::async_runtime::spawn(updater::check_update(app.handle().clone()));
    qr_reader::init(app.handle());
    hotkey::init(app.handle());
    i18n::initialize(app.handle());
    tray::init(app)?;
    platform::init(app.handle())?;
    info!("Setup completed");
    Ok(())
}
