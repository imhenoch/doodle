extern crate gio;
extern crate gtk;
use gio::prelude::*;
use std::env::args;

mod automatas;
mod input_formatter;
mod symbol_table;
mod syntax;
mod ui;

fn main() {
    let application =
        gtk::Application::new("com.github.imhenoch.doodle", gio::ApplicationFlags::empty())
            .expect("Gtk initialization failed");

    application.connect_startup(|app| {
        ui::app::build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
