extern crate gio;
extern crate gtk;

use gtk::prelude::*;
use gtk::{ApplicationWindow, Box, Button, Grid, Orientation, PackType, ScrolledWindow, TextView};

use automatas::input_processor;
use input_formatter::string_processor;

pub fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Doodle");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(1200, 900);

    let h_container = Box::new(Orientation::Horizontal, 5);

    let v_container = Box::new(Orientation::Vertical, 10);
    v_container.set_homogeneous(false);

    let text = text_view(&h_container);
    let (table, errors) = log(&h_container);
    h_container.set_homogeneous(true);
    v_container.add(&h_container);

    v_container.set_child_packing(&h_container, true, true, 5, PackType::Start);

    btn(&v_container, &text, &table, &errors);

    window.add(&v_container);
    window.show_all();
}

fn text_view(container: &Box) -> TextView {
    let scroll = ScrolledWindow::new(None, None);
    let text = TextView::new();

    scroll.add(&text);
    container.add(&scroll);
    text
}

fn log(container: &Box) -> (Grid, TextView) {
    let v_container = Box::new(Orientation::Vertical, 10);
    let scroll = ScrolledWindow::new(None, None);
    let table = Grid::new();
    let errors = TextView::new();

    scroll.add(&errors);
    v_container.add(&table);
    v_container.add(&scroll);

    v_container.set_homogeneous(true);
    container.add(&v_container);

    (table, errors)
}

fn btn(container: &Box, text: &TextView, table: &Grid, errors: &TextView) {
    let compile = Button::new_with_label("Compile");
    let text_clone = text.clone();
    let table_clone = table.clone();
    let errors_clone = errors.clone();
    container.add(&compile);

    compile.connect_clicked(move |_| {
        let buffer = text_clone.get_buffer().unwrap();
        let error_buffer = errors_clone.get_buffer().unwrap();
        let start = buffer.get_iter_at_offset(0);
        let end = buffer.get_end_iter();
        let text = buffer.get_text(&start, &end, true);
        let input = string_processor::transform_string_to_collection(text.expect(""));
        let symbols = input_processor::get_symbols(input);
        for symbol in symbols {
            println!("{}", symbol);
        }

        error_buffer.set_text("Some test");
        errors_clone.set_buffer(&error_buffer);
    });
}
