extern crate gio;
extern crate glib;
extern crate gtk;
extern crate htmlescape;
extern crate pango;

extern crate regex;
extern crate reqwest;
extern crate select;

mod ui;
mod lib;

use std::process;

fn main() {
    match ui::run_app() {
        Ok(_) => {}
        Err(e) => {
            eprint!("Failed to run app: {}", e);
            process::exit(1);
        }
    }
}
