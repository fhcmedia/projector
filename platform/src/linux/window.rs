use gtk4 as gtk;
use gtk::ApplicationWindowBuilder;
use gtk::ApplicationWindow;
use crate::{AbstractWindow, Window};

impl AbstractWindow<ApplicationWindow> for Window<ApplicationWindowBuilder> {
    fn create() -> Window<ApplicationWindowBuilder> {
        Window {
            window: ApplicationWindow::builder(),
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


