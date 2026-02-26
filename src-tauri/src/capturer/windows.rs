use anyhow::Result;
use windows::core::HSTRING;
use windows::Foundation::Uri;
use windows::System::Launcher;

const WIN_CAPTURE_URI: HSTRING = "ms-screenclip://capture/image?rectangle&".into();

pub async fn capture(app: &AppHandle) -> Result<Option<DynamicImage>> {
    Launcher::LaunchUri(Uri::CreateUri(&WIN_CAPTURE_URI));
    unimplemented!("This application is not supported on this platform :(");
}
