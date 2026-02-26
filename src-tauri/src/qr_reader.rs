use std::sync::atomic::Ordering;

use crate::{i18n, screenshot, AppState};
use anyhow::Error;
use log::{error, info, trace, warn};
use tauri::{AppHandle, Manager};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_opener::OpenerExt;
use url::Url;

#[derive(Debug)]
pub enum ScanResponse {
    Success(String),
    Canceled,
    NotFound,
    CaptureError(Error),
    QRDecodeError(&'static str),
    EncodingError,
}

async fn scan_qr(app: &AppHandle) -> ScanResponse {
    let image = screenshot::capture(app).await;
    let image = match image {
        Ok(image) => match image {
            Some(image) => image,
            None => return ScanResponse::Canceled,
        },
        Err(error) => {
            return ScanResponse::CaptureError(error);
        }
    };
    let mut scanner = zbar_rust::ZBarImageScanner::new();
    let img_width = image.width();
    let img_height = image.height();
    let results = scanner.scan_y800(image.to_luma8().into_raw(), img_width, img_height);
    let mut results = if let Err(err) = results {
        return ScanResponse::QRDecodeError(err);
    } else {
        results.unwrap()
    };
    if results.is_empty() {
        return ScanResponse::NotFound;
    }
    match try_decode_to_utf8(results.remove(0).data) {
        Some(content) => ScanResponse::Success(content),
        None => ScanResponse::EncodingError,
    }
}

pub fn process_qr(app: &AppHandle) {
    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        let capturing = &app_handle.state::<AppState>().capturing;
        if capturing.load(Ordering::Relaxed) {
            trace!("Multiple activation");
            return;
        }
        capturing.store(true, Ordering::Relaxed);
        process_qr_inner(&app_handle).await;
        capturing.store(false, Ordering::Relaxed);
    });
}

async fn process_qr_inner(app_handle: &AppHandle) {
    let open_browser = app_handle
        .state::<AppState>()
        .open_browser
        .load(Ordering::Relaxed);
    match scan_qr(app_handle).await {
        ScanResponse::Success(content) => {
            info!("Success! '{content}'");
            if open_browser && is_url(&content) {
                open_as_url(app_handle, &content);
            } else {
                open_as_text(app_handle, &content);
            }
        }
        ScanResponse::Canceled => {
            trace!("Canceled");
        }
        ScanResponse::NotFound => {
            trace!("Not found");
            notificate(
                app_handle,
                Some(&i18n::translate(app_handle, "notification.notfound")),
                None,
            );
        }
        ScanResponse::CaptureError(error) => {
            error!("Capture error! {error}");
            notificate(
                app_handle,
                Some(&i18n::translate(app_handle, "notification.capture_error")),
                None,
            );
        }
        ScanResponse::QRDecodeError(error) => {
            error!("Decode error! {}", error);
            notificate(
                app_handle,
                Some(&i18n::translate(app_handle, "notification.qr_decode_error")),
                None,
            );
        }
        ScanResponse::EncodingError => {
            error!("Encoding error!");
            notificate(
                app_handle,
                Some(&i18n::translate(app_handle, "notification.encoding_error")),
                None,
            );
        }
    };
}

fn try_decode_to_utf8(data: Vec<u8>) -> Option<String> {
    match String::from_utf8(data) {
        Ok(text) => Some(text),
        Err(_) => None,
    }
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
