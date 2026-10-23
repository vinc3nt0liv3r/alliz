use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    // Create a application
    let app = Application::builder()
        .application_id("org.alliz.viewer")
        .build();

    // Connect the "activate" signal to create the main window
    app.connect_activate(|app| {
        // Create the main application window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("alliz - Document Viewer")
            .default_width(800)
            .default_height(600)
            .build();

        // Show the window
        window.present();
    });

    // Run the application
    app.run();
}
