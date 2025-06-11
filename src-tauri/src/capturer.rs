use image::DynamicImage;
use std::error::Error;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

#[cfg(target_os = "macos")]
pub async fn capture(app: &AppHandle) -> Result<Option<DynamicImage>, Box<dyn Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let img_file = tmp_dir.path().join("scan.png");

    app.shell()
        .command("screencapture")
        .arg("-i")
        .arg(img_file.as_os_str())
        .output()
        .await?;
    Ok(image::open(img_file).ok())
}

#[allow(unused)]
const WIN_CAPTURE_URI: &str = "ms-screenclip://capture/image?rectangle&";

#[cfg(target_os = "windows")]
pub async fn capture(app: &AppHandle) -> Result<Option<DynamicImage>, Box<dyn Error>> {
    panic!("This application is not supported on this platform :(");
}
