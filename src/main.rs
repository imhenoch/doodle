extern crate gio;
extern crate gtk;
use gio::prelude::*;
use std::env::args;

mod lexer;
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
