extern crate gio;
extern crate gtk;

use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Box, Button, Grid, Label, Orientation, PackType, ScrolledWindow, TextView,
};

use automatas::{input_processor, Category, DataType, Lexem};
use input_formatter::string_processor;
use symbol_table::get_symbol_table;

pub fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("Doodle");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(1200, 700);

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
    let errors_scroll = ScrolledWindow::new(None, None);
    let table_scroll = ScrolledWindow::new(None, None);
    let table = Grid::new();
    let errors = TextView::new();

    errors.set_editable(false);

    table_scroll.add(&table);
    errors_scroll.add(&errors);
    v_container.add(&table_scroll);
    v_container.add(&errors_scroll);
    table.set_column_homogeneous(true);

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
        let (symbol_table, errors) = get_symbol_table(&symbols);
        let mut error = String::new();

        let mut i = 2;
        for child in table_clone.get_children() {
            table_clone.remove(&child);
        }
        let (token, data_type, data_size, category, scope) = grid_headers();
        table_clone.attach(&token, 0, 0, 1, 1);
        table_clone.attach(&data_type, 1, 0, 1, 1);
        table_clone.attach(&data_size, 2, 0, 1, 1);
        table_clone.attach(&category, 3, 0, 1, 1);
        table_clone.attach(&scope, 4, 0, 1, 1);
        for (_token, symbol) in symbol_table {
            let (token, data_type, data_size, category, scope) = grid_items(symbol);
            table_clone.attach(&token, 0, i, 1, 1);
            table_clone.attach(&data_type, 1, i, 1, 1);
            table_clone.attach(&data_size, 2, i, 1, 1);
            table_clone.attach(&category, 3, i, 1, 1);
            table_clone.attach(&scope, 4, i, 1, 1);
            i += 1;
        }
        for err in errors {
            error.push_str(&format!("{}\n", err.as_str()));
        }
        table_clone.show_all();

        error_buffer.set_text(error.as_str());
        errors_clone.set_buffer(&error_buffer);
    });
}

fn grid_headers() -> (Label, Label, Label, Label, Label) {
    let token = Label::new("TOKEN");
    let data_type = Label::new("DATA TYPE");
    let data_size = Label::new("SIZE");
    let category = Label::new("CATEGORY");
    let scope = Label::new("SCOPE");

    (token, data_type, data_size, category, scope)
}

fn grid_items(symbol: &Lexem) -> (Label, Label, Label, Label, Label) {
    let token = Label::new(symbol.token.as_str());
    let data_type = Label::new(match symbol.data_type {
        DataType::NONE => "-",
        DataType::INT => "int",
        DataType::FLOAT => "float",
        DataType::BOOL => "bool",
        DataType::CHAR => "char",
        DataType::STR => "str",
    });
    let data_size = Label::new(symbol.size.to_string().as_str());
    let category = Label::new(match symbol.category {
        Category::NONE => "-",
        Category::KEYWORD => "keyword",
        Category::ID => "id",
        Category::OPERATOR => "operator",
        Category::VALUE => "value",
        Category::LIMITER => "limiter",
    });
    let scope = Label::new(symbol.scope.as_str());

    (token, data_type, data_size, category, scope)
}
