use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tauri_plugin_updater::UpdaterExt;

use crate::platform;

pub async fn check_update(app: AppHandle) {
    let update_available = app
        .updater()
        .unwrap()
        .check()
        .await
        .is_ok_and(|v| v.is_some());
    let force_open = cfg!(debug_assertions) && option_env!("ALWAYS_OPEN_UPDATE_WINDOW").is_some();
    if update_available || force_open {
        create_window(&app);
    }
}

fn create_window(app: &AppHandle) {
    let window = WebviewWindowBuilder::new(app, "updater", WebviewUrl::App("index.html".into()))
        .inner_size(350., 150. + platform::window_decoration_height())
        .focused(true)
        .resizable(cfg!(debug_assertions))
        .title("Updater")
        .closable(false)
        .build()
        .unwrap();
    let cloned_window = window.clone();
    cloned_window.on_window_event(move |e| match e {
        WindowEvent::CloseRequested { api, .. } => {
            window.hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    });
}
