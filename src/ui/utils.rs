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
use gdk_pixbuf::Pixbuf;
use glib;
use gtk::prelude::*;

use std::sync::mpsc::{channel, Receiver};
use std::cell::RefCell;
use std::path::Path;
use std::thread;

use ui::widgets::video_wide;
use lib::{downloader, youtube};

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
    #[allow(unknown_lints, type_complexity)]
    static GLOBAL: RefCell<Option<(
        gtk::Viewport,
        Receiver<Option<Vec<youtube::Video>>>,
    )>> = RefCell::new(None);
    #[allow(unknown_lints, type_complexity)]
    static THUMBNAIL: RefCell<Option<(
        Vec<gtk::Image>,
        Receiver<Option<String>>,
    )>> = RefCell::new(None);
}

pub fn refresh_trending(viewport: &gtk::Viewport) {
    let children = viewport.get_children();
    for child in children {
        child.destroy();
    }
    let spinner = gtk::Spinner::new();
    spinner.show();
    spinner.start();
    viewport.add(&spinner);

    let (tx, rx) = channel();
    GLOBAL.with(clone!(viewport => move |global| {
        *global.borrow_mut() = Some((viewport, rx));
    }));

    thread::spawn(move || {
        let trending_videos = youtube::get_trending_videos();
        // Refresh if not busy
        if tx.send(trending_videos).is_ok() {
            glib::idle_add(refresh_trending_view);
        }
    });
}

fn refresh_trending_view() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref viewport, ref rx)) = *global.borrow() {
            if let Ok(trending_videos) = rx.try_recv() {
                match trending_videos {
                    Some(videos) => {
                        let listbox = gtk::ListBox::new();
                        listbox.set_size_request(720, 0);
                        listbox.set_halign(gtk::Align::Center);

                        let mut thumbnails: Vec<gtk::Image> = vec![];
                        let mut ids: Vec<String> = vec![];
                        for mut video in &videos {
                            let video_widget = video_wide::new(
                                &video.title,
                                &video.author,
                                &video.views,
                                &video.duration,
                            );
                            listbox.insert(&video_widget.video, -1);
                            ids.push(video.id.to_owned());
                            thumbnails.push(video_widget.thumbnail)
                        }
                        let spinner = viewport.get_children();
                        spinner[0].destroy();
                        listbox.show_all();
                        viewport.add(&listbox);
                        load_thumbnails(thumbnails, ids);
                    }
                    None => {
                        // If there's a network error such as no internet connection.
                        let label = gtk::Label::new("");
                        label.set_markup(
                            "<span size=\"x-large\">Couldn't fetch data.\
                             \nAre you connected to the internet?</span>",
                        );
                        let spinner = viewport.get_children();
                        spinner[0].destroy();
                        label.show();
                        viewport.add(&label);
                    }
                };
            }
        }
    });
    glib::Continue(false)
}

// Caches image at $HOME/.config/$NAME_NOCAPS/cache/images/$ID.jpg
fn load_thumbnails(images: Vec<gtk::Image>, ids: Vec<String>) {
    let (tx, rx) = channel();
    THUMBNAIL.with(move |thumbnail| {
        *thumbnail.borrow_mut() = Some((images, rx));
    });
    thread::spawn(move || {
        let cache_dir = format!("{}/cache/images", downloader::get_config_dir());
        for (i, id) in ids.iter().enumerate() {
            let file = format!("{}.jpg", id);
            let file_dir = format!("{}/{}", &cache_dir, &file);
            if !Path::new(&file_dir).is_file() {
                let url = format!("https://i.ytimg.com/vi/{}/mqdefault.jpg", id);
                downloader::download_to(&cache_dir, &file, &url);
            }
            tx.send(Some(file_dir.clone()))
                .expect("Could not send data to thread.");
            glib::idle_add(move || {
                THUMBNAIL.with(|thumbnail| {
                    if let Some((ref images, ref rx)) = *thumbnail.borrow() {
                        if let Ok(file_dir) = rx.try_recv() {
                            let file_dir = file_dir.unwrap();
                            let pixbuf =
                                Pixbuf::new_from_file_at_size(&file_dir, 240, 135).unwrap();
                            images[i].set_from_pixbuf(&pixbuf);
                        }
                    }
                });
                glib::Continue(false)
            });
        }
    });
}
