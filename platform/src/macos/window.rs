use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::Window as NSWindow;
use crate::{AbstractApplication, Application, AbstractWindow, Window};

// Implement App Delegation
impl AppDelegate for Application {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(
            self.properties.default_width,
            self.properties.default_height
        );
        self.window.set_title(self.properties.title.as_str());
        self.window.show();
    }
}

// Implement AbstractApplication
impl AbstractApplication<NSWindow> for Application {
    fn create() -> Application {
        Self {
            properties: Default::default()
        }
    }

    fn name(&mut self, name: &str) -> Self {
        self.properties.name = name.to_string();
        self.clone()
    }

    fn application_id(&mut self, id: &str) -> Application {
        self.properties.app_id = id.to_string();
        self.clone()
    }

    fn run(&self, window: Window<NSWindow>) {
        App::new(self.properties.app_id.as_str(), window).run();
    }
}


// Implement AbstractWindow
impl AbstractWindow<NSWindow> for Window<NSWindow> {
    fn create() -> Window<NSWindow> {
        Window {
            window: NSWindow::default(),
            properties: Default::default()
        }
    }

    fn title(&mut self, title: &str) -> Self {
        self.properties.title = title.to_string();
        self
    }

    fn default_width(mut self, width: i32) -> Self {
        self.properties.default_width = width;
        self
    }

    fn default_height(mut self, height: i32) -> Self {
        self.properties.default_height = height;
        self
    }

    fn set_width(mut self, width: i32) -> Self {
        self.properties.width = width;
        self
    }

    fn set_height(mut self, height: i32) -> Self {
        self.properties.height = height;
        self
    }
}
