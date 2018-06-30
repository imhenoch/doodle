extern crate gio;
extern crate gtk;

use gtk::prelude::*;
use gtk::{ApplicationWindow, Box, Button, Orientation, PackType, ScrolledWindow, TextView};

pub fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Doodle");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(700, 500);

    let container = Box::new(Orientation::Vertical, 10);
    let scroll = ScrolledWindow::new(None, None);
    let text = TextView::new();
    let btn2 = Button::new_with_label("Btn 2");

    scroll.add(&text);
    container.set_homogeneous(false);
    container.add(&scroll);
    container.add(&btn2);

    container.set_child_packing(&scroll, true, true, 5, PackType::Start);
    window.add(&container);
    window.show_all();
}
