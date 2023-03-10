// Libraries
use platform::{Application, Window};

// Main
fn main() {
    // Create application
    let app = Application::create()
        .application_id("com.fhcc.projector");

    // Create window
    let window = Window::create()
        .default_width(400)
        .default_hieght(400)
        .title("FHCC Projector");

    // NOTE: Any windows/tabs/lists/any content will follow 
    // the same style of creation as 'app' and 'window'.
    
    // Start app
    app.connect(window);
}
