use cacao::appkit::window::Window as NSWindow;
use crate::{AbstractWindow, Window};

// Implement AbstractWindow
impl AbstractWindow<NSWindow> for Window<NSWindow> {
    fn create() -> Window<NSWindow> {
        Window {
            window: NSWindow::default(),
            properties: Default::default()
        }
    }

    fn title(mut self, title: &str) -> Self {
        self.properties.title = title.to_string();
        return self
    }

    fn default_width(mut self, width: i32) -> Self {
        self.properties.default_width = width;
        return self
    }

    fn default_height(mut self, height: i32) -> Self {
        self.properties.default_height = height;
        return self
    }

    fn set_width(mut self, width: i32) -> Self {
        self.properties.width = width;
        return self
    }

    fn set_height(mut self, height: i32) -> Self {
        self.properties.height = height;
        return self
    }
}
