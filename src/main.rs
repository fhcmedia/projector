// Mod Platform
#[cfg(target_os = "macos")]
use platform::macos as platform;
#[cfg(target_os = "windows")]
use platform::windows as platform;
#[cfg(target_os = "linux")]
use platform::linux as platform;


fn main() {
    platform::window::spawn();
}


