// Libraries
#[cfg(target_os = "macos")]
use platform::macos::{ 
    application, keybinds, window 
};
#[cfg(target_os = "windows")]
use platform::windows::{ 
    application, keybinds, window 
};
#[cfg(target_os = "linux")]
use platform::linux::{ 
    application, keybinds, window 
};
use database;
use errors;
use logging;
use modules;


// Main
fn main() {
    window::spawn();
}


