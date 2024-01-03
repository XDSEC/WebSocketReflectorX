use adw::{Application, ApplicationWindow};
use gtk::glib;
use gtk::prelude::*;

const APP_ID: &str = "tech.woooo.wsrx";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(startup);
    app.connect_activate(build_ui);

    app.run()
}

fn startup(_app: &Application) {}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(1200)
        .default_height(700)
        .title("WebSocket Reflector X")
        .build();

    // Present window
    window.present();
}
