use anyhow::Result;
use image::DynamicImage;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

pub async fn capture(app: &AppHandle) -> Result<Option<DynamicImage>> {
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
