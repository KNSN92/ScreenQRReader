use std::error::Error;

use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::{qr_reader::process_qr, AppState};

pub fn register_capture_hotkey(app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let app_state = app.state::<AppState>();
    let shortcut = *app_state
        .read_qr_shortcut
        .lock()
        .expect("Poisoned global shortcut");
    app.global_shortcut()
        .on_shortcut(shortcut, move |app, event_shortcut, event| {
            if shortcut == *event_shortcut && event.state() == ShortcutState::Pressed {
                process_qr(app);
            }
        })?;
    Ok(())
}

pub fn unregister_capture_hotkey(app: &AppHandle) -> Result<(), Box<dyn Error>> {
    let app_state = app.state::<AppState>();
    let shortcut = *app_state
        .read_qr_shortcut
        .lock()
        .expect("Poisoned global shortcut");
    app.global_shortcut().unregister(shortcut)?;
    Ok(())
}
