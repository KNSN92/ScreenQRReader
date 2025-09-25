use std::{
    error::Error,
    sync::{atomic::Ordering, Arc},
};
use tauri::{
    include_image,
    menu::{CheckMenuItem, MenuBuilder, MenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, AppHandle, Manager,
};
use tauri_plugin_store::{Store, StoreExt};

use crate::{
    i18n,
    qr_reader::process_qr,
    hotkey::{register_capture_hotkey, unregister_capture_hotkey},
    AppState,
};

const OPEN_BROWSER_PREFERENCE_DEFAULT: bool = true;
const ENABLE_CAPTURE_HOTKEY_PREFERENCE_DEFAULT: bool = false;

pub fn setup_tray(app: &App) -> Result<(), Box<dyn Error>> {
    let scan_i = MenuItem::with_id(
        app,
        "scan",
        i18n::translate(app.handle(), "tray.scan"),
        true,
        Some("Command+Shift+1"),
    )?;

    let preference_open_browser = get_stored_bool_value(app.handle(), "open_browser", OPEN_BROWSER_PREFERENCE_DEFAULT);
    app.state::<AppState>()
        .open_browser
        .store(preference_open_browser, Ordering::Relaxed);
    let open_browser_i = CheckMenuItem::with_id(
        app,
        "open_browser",
        i18n::translate(app.handle(), "tray.open_browser"),
        true,
        preference_open_browser,
        None::<&str>,
    )?;
    let preference_enable_hotkey = get_stored_bool_value(app.handle(),"enable_hotkey", ENABLE_CAPTURE_HOTKEY_PREFERENCE_DEFAULT);
    if preference_enable_hotkey {
        register_capture_hotkey(app.handle()).unwrap();
    }
    let enable_hotkey_i = CheckMenuItem::with_id(
        app,
        "enable_hotkey",
        i18n::translate(app.handle(), "tray.enable_hotkey"),
        true,
        preference_enable_hotkey,
        None::<&str>,
    )?;
    let preferences_i = Submenu::with_items(
        app,
        i18n::translate(app.handle(), "tray.preferences"),
        true,
        &[&open_browser_i, &enable_hotkey_i],
    )?;

    let menu = MenuBuilder::new(app)
        .item(&scan_i)
        .separator()
        .item(&preferences_i)
        .separator()
        .about_with_text(i18n::translate(app.handle(), "tray.about"), None)
        .separator()
        .quit_with_text(i18n::translate(app.handle(), "tray.quit"))
        .build()?;

    TrayIconBuilder::new()
        .icon(include_image!("tray-icon.png"))
        .icon_as_template(true)
        .on_tray_icon_event(|tray, event| tray_icon_event(tray, event))
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "scan" => process_qr(app),
            "open_browser" => {
                if let Ok(store) = get_store(app) {
                    let preference_open_browser = open_browser_i
                        .is_checked()
                        .unwrap_or(OPEN_BROWSER_PREFERENCE_DEFAULT);
                    app.state::<AppState>()
                        .open_browser
                        .store(preference_open_browser, Ordering::Relaxed);
                    store.set(
                        "open_browser",
                        open_browser_i
                            .is_checked()
                            .unwrap_or(OPEN_BROWSER_PREFERENCE_DEFAULT),
                    );
                }
            }
            "enable_hotkey" => {
                let enable_hotkey = enable_hotkey_i
                    .is_checked()
                    .unwrap_or(ENABLE_CAPTURE_HOTKEY_PREFERENCE_DEFAULT);
                if let Ok(store) = get_store(app) {
                    store.set("enable_hotkey", enable_hotkey);
                }
                if enable_hotkey {
                    register_capture_hotkey(app).unwrap();
                } else {
                    unregister_capture_hotkey(app).unwrap();
                }
            }
            _ => {}
        })
        .show_menu_on_left_click(false)
        .build(app)?;
    Ok(())
}

fn tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } => {
            process_qr(tray.app_handle());
        }
        _ => {}
    }
}

fn get_store<R: tauri::Runtime>(app: &AppHandle<R>) -> tauri_plugin_store::Result<Arc<Store<R>>> {
    app.store("config.json")
}

fn get_stored_bool_value<R: tauri::Runtime>(app: &AppHandle<R>, key: &str, default: bool) -> bool {
    if let Ok(store) = get_store(app) {
        store
            .get(key)
            .map_or(default, |v| v.as_bool().unwrap_or(default))
    } else {
        default
    }
}