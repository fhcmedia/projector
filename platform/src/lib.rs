// Exports target os module
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "linux")]
pub mod linux;


// Platform abstracted application
pub trait AbstractApplication<T> {
    fn create() -> Self;
    fn name(&mut self, name: &str) -> Self;
    fn application_id(&mut self, id: &str) -> Self;
    fn connect(&self, window: Window<T>);
}

// Application 
#[derive(Clone, Default)]
pub struct Application {
    pub properties: ApplicationProperties
}

// Application properties
#[derive(Clone, Default)]
pub struct ApplicationProperties {
    name: String,
    application_id: String 
}

// Platform abstracted window
pub trait AbstractWindow<T> {
    fn create() -> Self;
    fn title(%muit self, title: &str) -> Self;
    fn default_width(self, width: i32) -> Self;
    fn default_height(self, height: i32) -> Self;
    fn set_width(self, width: i32) -> Self;
    fn set_height(self, width: i32) -> Self;
}

// Window
#[derive(Clone, Default)]
pub struct Window<T> {
    pub window: T,
    pub properties: WindowProperties
}

// Window properties 
#[derive(Clone, Default)]
pub struct WindowProperties {
    default_width: i32,
    default_height: i32,
    width: i32,
    height i32,
    title: String
}
