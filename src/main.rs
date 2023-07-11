// Libraries
use platform::{AbstractApplication, AbstractWindow, Application, Window};
use log::{debug, warn, error, info};


// Main
fn main() {
    // Init logger
    env_logger::init();
    info!("Plz work");

    // Create application
    let app = Application::create()
        .application_id("com.projector");

    // Create window
    let window = Window::create()
        .default_width(400)
        .default_height(400)
        .title("Projector");

    // NOTE: Any windows/tabs/lists/any content will follow 
    // the same style of creation as 'app' and 'window'.
    
    // Start app
    app.run(window);
    info!("Test log msg...");
}
