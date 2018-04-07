#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
mod utils;
mod static_resources;
mod app;
mod headerbar;
mod views;
mod widgets;
mod preferences;

extern crate gdk;
extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate htmlescape;
extern crate pango;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate hyper;
extern crate regex;
extern crate reqwest;
extern crate select;

extern crate sentinel_api;

use std::process;

pub const NAME: &str = "Sentinel";
pub const NAME_NOCAPS: &str = "sentinel";
pub const TAGLINE: &str = "Stream videos from the web.";

fn main() {
    gtk::init().expect("Error initializing gtk.");
    static_resources::init().expect("Something went wrong.");
    match app::run_app() {
        Ok(_) => {}
        Err(e) => {
            eprint!("Failed to run app: {}", e);
            process::exit(1);
        }
    }
}
