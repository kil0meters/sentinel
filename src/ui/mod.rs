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
    let trending_viewport: gtk::Viewport = builder.get_object("trending_viewport").unwrap();
    let trending_spinner: gtk::Spinner = builder.get_object("trending_spinner").unwrap();

    let (tx, rx) = channel();

    thread_local! (
        static LOADING: RefCell<Option<(gtk::Spinner, gtk::Viewport, Receiver<Vec<youtube::Video>>)>> = RefCell::new(None);
    );

    LOADING.with(move |loading| {
        *loading.borrow_mut() = Some((trending_spinner, trending_viewport, rx));
    });

    thread::spawn(move || {
        let trending_videos = youtube::get_trending_videos();
        tx.send(trending_videos)
            .expect("couldn't send data to thread");
        glib::idle_add(move || {
            LOADING.with(|loading| {
                if let Some((ref trending_spinner, ref trending_viewport, ref rx)) = *loading.borrow() {
                    if let Ok(trending_videos) = rx.try_recv() {
                        let trending_builder = gtk::Builder::new_from_string(include_str!("../../data/ui/trending_view.ui"));
                        let trending_listbox: gtk::ListBox = trending_builder.get_object("trending_listbox").unwrap();

                        for i in 0..20 {
                            let video_widget = video::create_new_wide(&trending_videos[i].title,
                                                                      &trending_videos[i].author,
                                                                      &trending_videos[i].views);

                            trending_listbox.insert(&video_widget.video, -1);
                        }

                        trending_listbox.show_all();
                        trending_spinner.destroy();
                        trending_viewport.add(&trending_listbox);
                    }
                }
            });
            glib::Continue(false)
        });
    });

    main_window.set_title("youtube-client");
    search_revealer.set_reveal_child(false);

    trending_scrolled_window.connect_edge_reached(move |_, direction| {
        if direction == gtk::PositionType::Bottom {
            //trending::load_new_trending_videos(5);
            println!("loading 5 more videos");
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
