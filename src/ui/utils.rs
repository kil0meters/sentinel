//  Copyright (C) 2017  Kil0meters
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.

use gtk;
use glib;
use gtk::prelude::*;

use std::sync::mpsc::{channel, Receiver};
use std::cell::RefCell;
use std::thread;

use ui::widgets::video_wide;
use lib::youtube;

// http://gtk-rs.org/tuto/closures
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}


// Create a thread local storage key to transfer data.
thread_local! {
    #[allow(unknown_lints)]
    #[allow(type_complexity)]
    static GLOBAL: RefCell<Option<(
        gtk::Overlay,
        Receiver<Option<Vec<youtube::Video>>>,
    )>> = RefCell::new(None);
}

pub fn refresh_trending(overlay: &gtk::Overlay) {
    let children = overlay.get_children();
    for child in children {
        child.destroy();
    }
    let spinner = gtk::Spinner::new();
    spinner.show();
    spinner.start();
    overlay.add(&spinner);

    let (tx, rx) = channel();
    GLOBAL.with(clone!(overlay => move |global| {
        *global.borrow_mut() = Some((overlay, rx));
    }));

    thread::spawn(move || {
        let trending_videos = youtube::get_trending_videos();
        tx.send(trending_videos)
            .expect("couldn't send data to thread");
        // Refresh data in main thread.
        glib::idle_add(refresh_trending_view);
    });
}

fn refresh_trending_view() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref overlay, ref rx)) = *global.borrow() {
            if let Ok(trending_videos) = rx.try_recv() {
                match trending_videos {
                    Some(videos) => {
                        let listbox = gtk::ListBox::new();
                        listbox.set_size_request(720, 0);
                        listbox.set_halign(gtk::Align::Center);

                        for video in &videos {
                            let video_widget =
                                video_wide::new(&video.title, &video.author, &video.views);
                            listbox.insert(&video_widget, -1);
                        }
                        listbox.show_all();
                        overlay.add_overlay(&listbox);
                    }
                    None => {
                        // If there's a network error such as no internet connection.
                        let label = gtk::Label::new("");
                        label.set_markup(
                            "<span size=\"x-large\">Couldn't fetch data.\
                             \nAre you connected to the internet?</span>",
                        );
                        let spinner = overlay.get_children();
                        spinner[0].destroy();
                        label.show();
                        overlay.add(&label);
                    }
                };
            }
        }
    });
    glib::Continue(false)
}
