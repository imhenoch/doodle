extern crate gio;
extern crate gtk;

use gtk::prelude::*;

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Doodle");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(400, 300);

    window.show_all();
}
