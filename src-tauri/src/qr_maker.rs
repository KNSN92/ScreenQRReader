use std::sync::OnceLock;

use anyhow::Result;
use image::{ImageBuffer, ImageFormat, Luma, Rgb};
use log::error;
use rxing::{
    qrcode::{decoder::ErrorCorrectionLevel, encoder::qrcode_encoder},
    Exceptions,
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent};

static QR_CODE_MAKER_WINDOW: OnceLock<WebviewWindow> = OnceLock::new();

fn create_window(app: &AppHandle) -> Result<()> {
    let window = WebviewWindowBuilder::new(app, "qr_maker", WebviewUrl::App("index.html".into()))
        .title("QR Code Maker")
        .inner_size(
            1024.,
            768. + if cfg!(target_os = "macos") { 27.5 } else { 0. },
        )
        .resizable(cfg!(debug_assertions))
        .visible_on_all_workspaces(true)
        .center()
        .devtools(cfg!(debug_assertions))
        .build()?;
    let cloned_window = window.clone();
    cloned_window.on_window_event(move |e| match e {
        WindowEvent::CloseRequested { api, .. } => {
            window.hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    });
    QR_CODE_MAKER_WINDOW.set(cloned_window).unwrap();
    Ok(())
}

pub fn show_window(app: &AppHandle) -> Result<()> {
    if QR_CODE_MAKER_WINDOW.get().is_none() {
        create_window(app)?;
    }
    let window = QR_CODE_MAKER_WINDOW.get().unwrap();
    window.center()?;
    window.show()?;
    window.set_focus()?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SerdeEcLevel {
    L = 0,
    M = 1,
    Q = 2,
    H = 3,
}

impl From<SerdeEcLevel> for ErrorCorrectionLevel {
    fn from(value: SerdeEcLevel) -> Self {
        match value {
            SerdeEcLevel::L => ErrorCorrectionLevel::L,
            SerdeEcLevel::M => ErrorCorrectionLevel::M,
            SerdeEcLevel::Q => ErrorCorrectionLevel::Q,
            SerdeEcLevel::H => ErrorCorrectionLevel::H,
        }
    }
}

impl Into<SerdeEcLevel> for ErrorCorrectionLevel {
    fn into(self) -> SerdeEcLevel {
        match self {
            ErrorCorrectionLevel::L => SerdeEcLevel::L,
            ErrorCorrectionLevel::M => SerdeEcLevel::M,
            ErrorCorrectionLevel::Q => SerdeEcLevel::Q,
            ErrorCorrectionLevel::H => SerdeEcLevel::H,
            ErrorCorrectionLevel::Invalid => SerdeEcLevel::M, // default to M
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateQRCodePayload {
    text: String,
    eclevel: SerdeEcLevel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerateQRCodeResponse {
    Success { code: String, width: u32 },
    Error(String),
}

#[tauri::command]
pub fn generate_qrcode(payload: GenerateQRCodePayload) -> GenerateQRCodeResponse {
    let qrcode = qrcode_encoder::encode(&payload.text, payload.eclevel.into());
    let qrcode = match qrcode {
        Ok(qrcode) => qrcode,
        Err(e) => {
            error!("{e:?}");
            return match e {
                Exceptions::WriterException(msg) if msg.to_lowercase().contains("data too big") => {
                    GenerateQRCodeResponse::Error("DataTooLong".to_string())
                }
                Exceptions::IllegalArgumentException(msg) if msg == "version out of spec" => {
                    GenerateQRCodeResponse::Error("InvalidVersion".to_string())
                }
                Exceptions::FormatException(_) => {
                    GenerateQRCodeResponse::Error("InvalidCharacter".to_string())
                }
                _ => GenerateQRCodeResponse::Error("UnknownError".to_string()),
            };
        }
    };
    let matrix = match qrcode.getMatrix() {
        Some(matrix) => matrix,
        None => {
            error!("QRCode matrix is empty ");
            return GenerateQRCodeResponse::Error("UnknownError".to_string());
        }
    };
    let width = matrix.getWidth();
    let code = matrix
        .getArray()
        .into_iter()
        .flatten()
        .map(|c| if *c != 0 { "1" } else { "0" })
        .collect::<String>();
    GenerateQRCodeResponse::Success { code, width }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValidateQRCodeResponse {
    Valid,
    Invalid,
    InvalidImage,
    ScanError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateQRCodePayload {
    text: String,
    image: Vec<u8>,
}

#[tauri::command]
pub fn validate_qrcode(payload: ValidateQRCodePayload) -> ValidateQRCodeResponse {
    let qr_img = image::load_from_memory_with_format(&payload.image, ImageFormat::Png);
    let qr_img = match qr_img {
        Ok(img) => img.to_luma8(),
        Err(_) => return ValidateQRCodeResponse::InvalidImage,
    };
    test_qrcode(qr_img, payload.text)
}

const MARGIN_COLORS: [Rgb<u8>; 8] = [
    Rgb([0, 0, 0]),
    Rgb([255, 0, 0]),
    Rgb([0, 255, 0]),
    Rgb([0, 0, 255]),
    Rgb([255, 255, 0]),
    Rgb([0, 255, 255]),
    Rgb([255, 0, 255]),
    Rgb([255, 255, 255]),
];

const MARGIN: usize = 2; //

// fn test_qrcode_outer_colors<P: Pixel, Container>(
//     qr_img: ImageBuffer<P, Container>,
// ) -> ValidateQRCodeResponse {
//     for margin_color in MARGIN_COLORS {
//         let with_margin_qr_img =
//     }
// }

fn test_qrcode(qr_img: ImageBuffer<Luma<u8>, Vec<u8>>, text: String) -> ValidateQRCodeResponse {
    let (img_width, img_height) = qr_img.dimensions();
    let result = rxing::helpers::detect_in_luma(qr_img.into_raw(), img_width, img_height, None);
    let result = match result {
        Ok(result) => result,
        Err(Exceptions::NotFoundException(_)) => return ValidateQRCodeResponse::Invalid,
        Err(_) => return ValidateQRCodeResponse::ScanError,
    };
    let qr_data = String::from_utf8(result.getRawBytes().to_vec());
    let qr_data = match qr_data {
        Ok(qr_data) => qr_data,
        Err(_) => return ValidateQRCodeResponse::Invalid,
    };
    if qr_data == text {
        ValidateQRCodeResponse::Valid
    } else {
        ValidateQRCodeResponse::Invalid
    }
}
