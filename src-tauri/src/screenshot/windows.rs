use anyhow::Result;
use windows::core::HSTRING;
use windows::ApplicationModel::DataTransfer::SharedStorageAccessManager;
use windows::Foundation::Uri;
use windows::System::Launcher;

const WIN_CAPTURE_URI: HSTRING = "ms-screenclip://capture/image?rectangle&enabledModes=RectangleSnip&redirect-uri=my-snip-protocol-test-app://response".into();

pub async fn capture(app: &AppHandle) -> Result<Option<DynamicImage>> {
    Launcher::LaunchUri(Uri::CreateUri(&WIN_CAPTURE_URI));
    unimplemented!("This application is not supported on this platform :(");
}
