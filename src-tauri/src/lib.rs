mod tray;
use std::sync::{atomic::AtomicBool, Mutex};

use log::info;
use tauri_plugin_global_shortcut::{Modifiers, Shortcut};
use tray::setup_tray;

mod capturer;
mod i18n;
mod qr_reader;
mod hotkey;

use tauri::{App, Manager};

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
            .build()
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
            setup(app);
            setup_tray(app)?;
            info!("Setup completed");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }

fn setup(app: &mut App) {
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
}
