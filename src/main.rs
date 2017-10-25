extern crate glib;
extern crate gtk;
extern crate htmlescape;
extern crate pango;

#[macro_use]
extern crate clap;

extern crate regex;
extern crate reqwest;
extern crate select;

mod ui;
mod lib;

fn main() {
    clap_app!(youtube_client =>
        (version: "0.1.0")
        (author: "kil0meters <kil0meters@protonmail.com>")
    ).get_matches();

    ui::launch();
}
