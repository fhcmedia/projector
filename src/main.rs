// Libraries
use platform::{AbstractApplication, AbstractWindow, Application, Window};
use log::{debug};

// Main
fn main() {
    env_logger::init();

    debug!("this is a test logging {}", "message");

    // Create application
    let app = Application::create()
        .application_id("com.projector");
        // Add version info to trait Application

    // Create window
    let window = Window::create()
        .default_width(400)
        .default_height(400)
        .title("Projector");

    // NOTE: Any windows/tabs/lists/any content will follow 
    // the same style of creation as 'app' and 'window'.
    
    // Start app
    app.run(window);
}
