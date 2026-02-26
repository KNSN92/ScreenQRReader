use anyhow::Result;
use qrcode::{types::QrError, EcLevel, QrCode};
use serde::{Deserialize, Serialize};
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
