use log::{info};
use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::Window as NSWindow;  
use crate::{AbstractApplication, Application, Window};

// Implement App Delegation
impl AppDelegate for Window<NSWindow> {
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
        return self.clone()
    }

    fn application_id(&mut self, id: &str) -> Application {
        self.properties.application_id = id.to_string();
        return self.clone()
    }

    fn run(&self, window: Window<NSWindow>) {
        App::new(self.properties.application_id.as_str(), window).run();
    }
}
