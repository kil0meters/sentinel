extern crate gtk;
extern crate pango;
extern crate glib;
extern crate htmlescape;

#[macro_use]
extern crate clap;

extern crate regex;
extern crate select;
extern crate reqwest;

mod ui;
mod lib;

fn main() {
    let matches = clap_app!(youtube_client =>
        (version: "0.1.0")
        (author: "kil0meters <kil0meters@protonmail.com>")
    ).get_matches();

    ui::launch();
}
