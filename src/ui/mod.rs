use glib;
use gtk;

#[macro_use]
mod trending;
mod video;

use std::sync::mpsc::{channel, Receiver};
use std::cell::RefCell;
use std::thread;

use gtk::prelude::*;

use lib::youtube;

use htmlescape::encode_minimal;

macro_rules! if_on_stack {
    ($stack_number:expr, $stack:ident, $data:block ) => {{
        let stack = &$stack;
        let visible_child = $stack.get_visible_child().unwrap();
        if stack.get_child_position(&visible_child) == $stack_number {
            $data
        }
    }}
}

pub fn launch() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let builder = gtk::Builder::new_from_string(include_str!("../../data/ui/main.ui"));

    let main_window: gtk::Window = builder.get_object("main_window").unwrap();

    // Search Button
    let search_revealer: gtk::Revealer   = builder.get_object("search_revealer").unwrap();
    let search_button: gtk::ToggleButton = builder.get_object("search_button").unwrap();
    // Buttons
    let refresh_button: gtk::Button = builder.get_object("refresh_button").unwrap();
    // Stack
    let main_window_stack: gtk::Stack = builder.get_object("stack").unwrap();
    // Trending
    let trending_scrolled_window: gtk::ScrolledWindow = builder.get_object("page0").unwrap();
    let trending_spinner: gtk::Spinner = builder.get_object("trending_spinner").unwrap();

    thread::spawn(|| {
        let trending_videos = youtube::get_trending_videos();
    });

    main_window.set_title("youtube-client");
    search_revealer.set_reveal_child(false);

    trending_scrolled_window.connect_edge_reached(move |_, direction| {
        if direction == gtk::PositionType::Bottom {
            trending::load_new_trending_videos(5);
        }
    });

    search_button.connect_clicked(move |_| {
        let state = search_revealer.get_reveal_child();

        if state == false {
            search_revealer.set_reveal_child(true);
        }
        else {
            search_revealer.set_reveal_child(false);
        }
    });

    refresh_button.connect_clicked(move |_| {
        if_on_stack!(0, main_window_stack, {
            // trending::update_trending();
        });
    });

    main_window.set_wmclass("youtube-client", "Youtube-client");
    main_window.show_all();

    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
