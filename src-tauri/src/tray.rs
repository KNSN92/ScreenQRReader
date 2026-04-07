use anyhow::Result;
use log::error;
use std::error::Error;
use tauri::{
    include_image,
    menu::{AboutMetadataBuilder, CheckMenuItem, MenuBuilder, MenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App,
};
use tauri_plugin_dialog::DialogExt;

use crate::{
    config::{load_cfg, store_cfg, ConfigKey, ConfigValue},
    hotkey::{register_capture_hotkey, unregister_capture_hotkey},
    i18n, qr_maker,
    qr_reader::process_qr,
};

pub fn setup_tray(app: &App) -> Result<(), Box<dyn Error>> {
    let scan_i = MenuItem::with_id(
        app,
        "scan",
        i18n::translate(app.handle(), "tray.scan"),
        true,
        Some("Command+Shift+1"),
    )?;
    let qrcode_maker_i = MenuItem::with_id(
        app,
        "qrcode_maker",
        i18n::translate(app.handle(), "tray.qrcode_maker"),
        true,
        Option::<&str>::None,
    )?;

    let open_browser = load_cfg(app.handle(), ConfigKey::OpenBrowser);
    let open_browser_i = CheckMenuItem::with_id(
        app,
        "open_browser",
        i18n::translate(app.handle(), "tray.open_browser"),
        true,
        open_browser,
        None::<&str>,
    )?;
    let read_qr_shortcut = load_cfg(app.handle(), ConfigKey::ReadQrShortcut);
    if read_qr_shortcut {
        register_capture_hotkey(app.handle()).unwrap();
    }
    let enable_hotkey_i = CheckMenuItem::with_id(
        app,
        "enable_hotkey",
        i18n::translate(app.handle(), "tray.enable_hotkey"),
        true,
        read_qr_shortcut,
        None::<&str>,
    )?;
    let preferences_i = Submenu::with_items(
        app,
        i18n::translate(app.handle(), "tray.preferences"),
        true,
        &[&open_browser_i, &enable_hotkey_i],
    )?;

    let about_metadata = AboutMetadataBuilder::new()
        .name(Some(i18n::translate(app.handle(), "screen_qr_reader")))
        .authors(Some(vec!["KNSN92".to_string()]))
        .copyright(Some(format!(
            "LGPL-2.1 License\n{}",
            i18n::translate(app.handle(), "tray.about.qrcode_brand")
        )))
        .build();

    let menu = MenuBuilder::new(app)
        .item(&scan_i)
        .item(&qrcode_maker_i)
        .separator()
        .item(&preferences_i)
        .separator()
        .about_with_text(
            i18n::translate(app.handle(), "tray.about"),
            Some(about_metadata),
        )
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
            "qrcode_maker" => {
                if let Err(e) = qr_maker::show_window(app) {
                    error!("Failed to open QR Code Maker: {e:?}");
                    app.dialog()
                        .message(i18n::translate(app, "dialog.cannot_open_qr_maker"));
                }
            }
            "open_browser" => {
                let open_browser_cfg = open_browser_i
                    .is_checked()
                    .map(ConfigValue::OpenBrowser)
                    .unwrap_or(ConfigValue::default_value(ConfigKey::OpenBrowser));
                store_cfg(app, open_browser_cfg);
            }
            "enable_hotkey" => {
                let enable_hotkey_cfg = enable_hotkey_i
                    .is_checked()
                    .map(ConfigValue::ReadQrShortcut)
                    .unwrap_or(ConfigValue::default_value(ConfigKey::ReadQrShortcut));
                if enable_hotkey_cfg == ConfigValue::ReadQrShortcut(true) {
                    register_capture_hotkey(app).unwrap();
                } else {
                    unregister_capture_hotkey(app).unwrap();
                }
                store_cfg(app, enable_hotkey_cfg);
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
