use std::sync::{atomic::AtomicBool, Mutex};

use anyhow::Result;
use log::info;
use tauri::{App, Manager};
use tauri_plugin_global_shortcut::{Modifiers, Shortcut};
use tray::setup_tray;

use crate::qr_maker::generate_qrcode;

mod hotkey;
mod i18n;
mod misc;
mod qr_maker;
mod qr_reader;
mod screenshot;
mod tray;

pub struct AppState {
    pub capturing: AtomicBool,
    pub read_qr_shortcut: Mutex<Shortcut>,
    pub open_browser: AtomicBool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            setup(app)?;
            setup_tray(app)?;
            info!("Setup completed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![generate_qrcode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(app: &mut App) -> Result<()> {
    app.manage(AppState {
        capturing: AtomicBool::new(false),
        read_qr_shortcut: Mutex::new(Shortcut::new(
            Some(Modifiers::union(Modifiers::META, Modifiers::SHIFT)),
            tauri_plugin_global_shortcut::Code::Digit1,
        )),
        open_browser: AtomicBool::new(false),
    });
    #[cfg(target_os = "macos")]
    {
        use tauri::ActivationPolicy;
        app.set_activation_policy(ActivationPolicy::Accessory);
        app.set_dock_visibility(false);
    }
    i18n::initialize(app.handle());
    qr_maker::create_window(app.handle())?;
    Ok(())
}
