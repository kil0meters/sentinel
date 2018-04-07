use gtk;
use glib;
use gtk::prelude::*;

use std::sync::mpsc::channel;
use std::thread;

use app::TRENDING;
use utils::load_thumbnails;
use widgets::video_wide;
use views::video_player;
use sentinel_api::youtube;

pub fn refresh(viewport: &gtk::Viewport) {
    let children = viewport.get_children();
    for child in children {
        child.destroy();
    }
    let spinner = gtk::Spinner::new();
    spinner.show();
    spinner.start();
    viewport.add(&spinner);

    let (tx, rx) = channel();
    TRENDING.with(clone!(viewport => move |trending| {
        *trending.borrow_mut() = Some((viewport, rx));
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
    TRENDING.with(|trending| if let Some((ref viewport, ref rx)) = *trending.borrow() {
        if let Ok(trending_videos) = rx.try_recv() {
            match trending_videos {
                Some(videos) => {
                    let listbox = gtk::ListBox::new();
                    listbox.set_size_request(720, 0);
                    listbox.set_halign(gtk::Align::Center);
                    listbox.set_activate_on_single_click(true);
                    // This shouldn't be needed, and doesn't work
                    // correctly.
                    // https://github.com/gtk-rs/gtk/issues/520
                    //listbox.connect_row_selected(
                    //    move |_, row| if let Some(row) = row.clone() {
                    //       row.activate();
                    //   },
                    //);

                    let mut thumbnails: Vec<gtk::Image> = vec![];
                    let mut ids: Vec<String> = vec![];
                    for mut video in &videos {
                        let video_widget: (gtk::ListBoxRow,
                            gtk::Image) = video_wide::new(
                            &video.title,
                            &video.author,
                            &video.views,
                            &video.duration,
                        );
                        listbox.insert(&video_widget.0, -1);
                        let id = video.id.clone();
                        video_widget.0.connect_activate(move |_| {
                            let id = id.to_owned();
                            video_player::watch(id);
                        });
                        ids.push(video.id.to_owned());
                        thumbnails.push(video_widget.1);
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
    });
    glib::Continue(false)
}

