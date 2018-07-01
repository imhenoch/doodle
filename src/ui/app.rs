extern crate gio;
extern crate gtk;

use gtk::prelude::*;
use gtk::{ApplicationWindow, Box, Button, Orientation, PackType, ScrolledWindow, TextView};

use lexer::string_processor;

pub fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Doodle");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(700, 500);

    let container = Box::new(Orientation::Vertical, 10);
    container.set_homogeneous(false);

    let text = text_view(&container);
    btn(&container, &text);

    window.add(&container);
    window.show_all();
}

fn text_view(container: &Box) -> TextView {
    let scroll = ScrolledWindow::new(None, None);
    let text = TextView::new();

    scroll.add(&text);
    container.add(&scroll);
    container.set_child_packing(&scroll, true, true, 5, PackType::Start);

    text
}

fn btn(container: &Box, text: &TextView) {
    let compile = Button::new_with_label("Compile");
    let text_clone = text.clone();
    container.add(&compile);

    compile.connect_clicked(move |_| {
        let buffer = text_clone.get_buffer().unwrap();
        let start = buffer.get_iter_at_offset(0);
        let end = buffer.get_end_iter();
        let text = buffer.get_text(&start, &end, true);
        let lexems = string_processor::transform_string_to_collection(text.expect(""));
    });
}
