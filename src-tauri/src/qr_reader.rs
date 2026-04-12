use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    config::{load_cfg, ConfigKey},
    i18n, screenshot,
};
use anyhow::{anyhow, Error};
use image::GenericImageView;
use log::{error, info, warn};
use rxing::Exceptions;
use tauri::{AppHandle, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;
use url::Url;

struct Capturing(AtomicBool);

pub fn init(app_handle: &AppHandle) {
    app_handle.manage(Capturing(AtomicBool::new(false)));
}

#[derive(Debug)]
pub enum ScanError {
    Canceled,
    NotFound,
    CaptureError(Error),
    QRDecodeError(Error),
    EncodingError,
}

async fn scan_qr(app: &AppHandle) -> Result<String, ScanError> {
    let image = screenshot::capture(app).await;
    let image = image
        .map(|image| image.ok_or(ScanError::Canceled))
        .map_err(|err| ScanError::CaptureError(err))
        .flatten()?;
    let (img_width, img_height) = image.dimensions();
    let result =
        rxing::helpers::detect_in_luma(image.to_luma8().into_raw(), img_width, img_height, None);
    result
        .map(|result| {
            String::from_utf8(result.getRawBytes().to_vec()).map_err(|_| ScanError::EncodingError)
        })
        .map_err(|err| match err {
            Exceptions::NotFoundException(_) => ScanError::NotFound,
            _ => ScanError::QRDecodeError(anyhow!(err)),
        })
        .flatten()
}

pub fn process_qr(app: &AppHandle) {
    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        let capturing = &app_handle.state::<Capturing>().0;
        if capturing.load(Ordering::Relaxed) {
            info!("Multiple activation");
            return;
        }
        capturing.store(true, Ordering::Relaxed);
        process_qr_inner(&app_handle).await;
        capturing.store(false, Ordering::Relaxed);
    });
}

async fn process_qr_inner(app_handle: &AppHandle) {
    let open_browser = load_cfg(app_handle, ConfigKey::OpenBrowser);
    let result = scan_qr(app_handle).await;
    let scan_err = match result {
        Ok(content) => {
            info!("Success! '{content}'");
            if open_browser && is_url(&content) {
                open_as_url(app_handle, &content);
            } else {
                open_as_text(app_handle, &content);
            }
            return;
        }
        Err(err) => err,
    };
    match scan_err {
        ScanError::Canceled => {
            info!("Canceled");
        }
        ScanError::NotFound => {
            info!("Not found");
            notificate(
                app_handle,
                Some(&i18n::translate(app_handle, "notification.notfound")),
                None,
            );
        }
        ScanError::CaptureError(error) => {
            error!("Capture error! {error}");
            notificate(
                app_handle,
                Some(&i18n::translate(app_handle, "notification.capture_error")),
                None,
            );
        }
        ScanError::QRDecodeError(error) => {
            error!("Decode error! {}", error);
            notificate(
                app_handle,
                Some(&i18n::translate(app_handle, "notification.qr_decode_error")),
                None,
            );
        }
        ScanError::EncodingError => {
            error!("Encoding error!");
            notificate(
                app_handle,
                Some(&i18n::translate(app_handle, "notification.encoding_error")),
                None,
            );
        }
    };
}

fn open_as_url(app_handle: &AppHandle, content: &str) {
    if app_handle.opener().open_url(content, None::<&str>).is_err() {
        warn!("Could not open this url {content}");
        notificate(
            app_handle,
            Some(&i18n::translate(app_handle, "notification.unopenable_url")),
            Some(content),
        );
    }
}

fn open_as_text(app_handle: &AppHandle, content: &str) {
    let result = app_handle
        .dialog()
        .message(i18n::translate_format(
            app_handle,
            "dialog.successful",
            content,
        ))
        .kind(MessageDialogKind::Info)
        .title(i18n::translate(app_handle, "screen_qr_reader"))
        .buttons(MessageDialogButtons::OkCancelCustom(
            i18n::translate(app_handle, "dialog.copy_clipboard").to_string(),
            i18n::translate(app_handle, "dialog.close").to_string(),
        ))
        .blocking_show();
    if result {
        app_handle
            .clipboard()
            .write_text(content)
            .expect(&i18n::translate(app_handle, "dialog.copy_clipboard_error"));
    }
}

fn is_url(content: &str) -> bool {
    Url::parse(content).is_ok()
}

fn notificate(app: &AppHandle, title: Option<&str>, body: Option<&str>) {
    let builder = app.notification().builder();
    let builder = match title {
        Some(title) => builder.title(title),
        None => builder,
    };
    let builder = match body {
        Some(body) => builder.body(body),
        None => builder,
    };
    builder.show().unwrap();
}
