pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

pub fn check_platform() {
    if is_macos() {
        return;
    }
    if is_windows() {
        panic!("This application is not supported on windows (but coming soon)");
    }
    panic!("This application is not supported on this platform :(");
}
