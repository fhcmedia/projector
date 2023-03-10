use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::Window;

#[derive(Default)]
struct Application {
    window: Window
}

impl AppDelegate for Application {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("Projector");
        self.window.show();
    }
}

pub fn spawn() {
    App::new("com.projector", Application::default()).run();
}
