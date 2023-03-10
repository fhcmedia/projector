use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::Window;
use crate::{AbstractWindow};

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

impl AbstractWindow for Window {
    fn spawn() {
        App::new("com.projector", Application::default()).run();
    }
}
