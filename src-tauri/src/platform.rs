#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "macos")]
mod macos {
    use anyhow::Result;
    use log::debug;
    use std::error::Error;
    use tauri::{ActivationPolicy, AppHandle};

    pub fn init(app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
        app_handle.set_activation_policy(ActivationPolicy::Accessory)?;
        app_handle.set_dock_visibility(false)?;
        debug!("macOS platform specific initialization completed");
        Ok(())
    }
}

#[cfg(target_os = "windows")]
pub use windows::*;
#[cfg(target_os = "windows")]
mod windows {
    use anyhow::Result;
    use log::{debug, info};
    use std::error::Error;
    use tauri::AppHandle;

    use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
    use warp::Filter;

    use crate::i18n;

    pub fn init(app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
        tauri::async_runtime::spawn(init_server(app_handle.clone()));
        debug!("Windows platform specific initialization completed");
        Ok(())
    }

    async fn init_server(app_handle: AppHandle) {
        let app_handle = &app_handle;
        let port = match portpicker::pick_unused_port() {
            Some(port) => port,
            None => {
                let _ = app_handle
                    .dialog()
                    .message(i18n::translate(app_handle, "dialog.no_port.message"))
                    .title(i18n::translate(app_handle, "dialog.no_port.title"))
                    .kind(MessageDialogKind::Error)
                    .blocking_show();
                app_handle.exit(1);
                return;
            }
        };
        info!("Selected port: {}", port);
        let root = warp::path!().map(|| "Screen QR Reader".to_string());
        warp::serve(root).run(([127, 0, 0, 1], port)).await;
        return;
    }
}

#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "linux")]
mod linux {
    use log::debug;
    use std::error::Error;
    use tauri::AppHandle;

    pub fn init(app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
        debug!("Linux platform specific initialization completed");
        Ok(())
    }
}
