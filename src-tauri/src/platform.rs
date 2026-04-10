#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "macos")]
mod macos {
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
    use log::debug;
    use std::error::Error;
    use tauri::AppHandle;

    pub fn init(app_handle: &AppHandle) -> Result<(), Box<dyn Error>> {
        debug!("Windows platform specific initialization completed");
        Ok(())
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
