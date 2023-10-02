use gtk4 as gtk;
use gtk::prelude::*;
use gtk::ApplicationWindowBuilder;
use gtk::Application as GTKApplication; // Renamed because we have an abstract 'Application' defined
use crate::{AbstractApplication, Application, Window};

impl AbstractApplication<ApplicationWindowBuilder> for Application {
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

    fn run(&self, w: Window<ApplicationWindowBuilder>) {
        let app = GTKApplication::builder()
            .application_id(self.properties.application_id.as_str())
            .build();
        app.connect_activate(move |app| {
            // Create the main window
            let window = w.window.clone()
                .title(w.properties.title.as_str())
                .default_width(w.properties.default_width)
                .default_height(w.properties.default_height)
                .application(app)
                .build();

        //window.show();
        });
        app.run();
    }
}




