use anyhow::Result;
use image::ImageFormat;
use log::debug;
use qrcode::{types::QrError, EcLevel, QrCode};
use serde::{de, Deserialize, Serialize};
use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

pub fn create_window(app: &AppHandle) -> Result<()> {
    let window = WebviewWindowBuilder::new(app, "qr_maker", WebviewUrl::App("index.html".into()))
        .title("QR Code Maker")
        // .resizable(false)
        .center()
        .build()?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SerdeEcLevel {
    L = 0,
    M = 1,
    Q = 2,
    H = 3,
}

impl From<SerdeEcLevel> for EcLevel {
    fn from(value: SerdeEcLevel) -> Self {
        match value {
            SerdeEcLevel::L => EcLevel::L,
            SerdeEcLevel::M => EcLevel::M,
            SerdeEcLevel::Q => EcLevel::Q,
            SerdeEcLevel::H => EcLevel::H,
        }
    }
}

impl Into<SerdeEcLevel> for EcLevel {
    fn into(self) -> SerdeEcLevel {
        match self {
            EcLevel::L => SerdeEcLevel::L,
            EcLevel::M => SerdeEcLevel::M,
            EcLevel::Q => SerdeEcLevel::Q,
            EcLevel::H => SerdeEcLevel::H,
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
    Success { code: String, width: usize },
    Error(String),
}

#[tauri::command]
pub fn generate_qrcode(payload: GenerateQRCodePayload) -> GenerateQRCodeResponse {
    let qr = QrCode::with_error_correction_level(payload.text.as_bytes(), payload.eclevel.into());
    match qr {
        Ok(qr) => GenerateQRCodeResponse::Success {
            code: qr
                .to_colors()
                .iter()
                .map(|c| c.select("1", "0"))
                .collect::<String>(),
            width: qr.width(),
        },
        Err(e) => match e {
            QrError::DataTooLong => GenerateQRCodeResponse::Error("DataTooLong".to_string()),
            QrError::InvalidCharacter => {
                GenerateQRCodeResponse::Error("InvalidCharacter".to_string())
            }
            QrError::InvalidEciDesignator => {
                GenerateQRCodeResponse::Error("InvalidEciDesignator".to_string())
            }
            QrError::InvalidVersion => GenerateQRCodeResponse::Error("InvalidVersion".to_string()),
            QrError::UnsupportedCharacterSet => {
                GenerateQRCodeResponse::Error("UnsupportedCharacterSet".to_string())
            }
        },
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValidateQRCodeResponse {
    Valid,
    Invalid,
    Error(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateQRCodePayload {
    data: Vec<u8>,
    image: Vec<u8>,
}

#[tauri::command]
pub fn validate_qrcode(payload: ValidateQRCodePayload) -> ValidateQRCodeResponse {
    let qr_img = image::load_from_memory_with_format(&payload.image, ImageFormat::Png);
    let qr_img = match qr_img {
        Ok(img) => img,
        Err(_) => return ValidateQRCodeResponse::Error("InvalidImage".to_string()),
    };
    let mut scanner = zbar_rust::ZBarImageScanner::new();
    let img_width = qr_img.width();
    let img_height = qr_img.height();
    let results = scanner.scan_y800(qr_img.to_luma8().into_raw(), img_width, img_height);
    let mut results = if let Err(_) = results {
        return ValidateQRCodeResponse::Error("ScanError".to_string());
    } else {
        results.unwrap()
    };
    if results.is_empty() {
        debug!("No QR code found in the image");
        return ValidateQRCodeResponse::Invalid;
    }
    let qr_data = results.remove(0).data;
    if qr_data != payload.data {
        debug!(
            "QR code data does not match. Expected: {:?}, Found: {:?}",
            payload.data, qr_data
        );
        return ValidateQRCodeResponse::Invalid;
    }
    ValidateQRCodeResponse::Valid
}
