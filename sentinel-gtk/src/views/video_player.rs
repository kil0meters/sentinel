use gtk;
use gdk;
use glib;
use gtk::prelude::*;

use app::VPLAYER;

use sentinel_api::youtube;

use std::thread;
use std::sync::{mpsc, Arc, Mutex};

use sentinel_api::utils::pretty_number;
use widgets::video;
use utils::load_thumbnails;

use htmlescape::encode_minimal;

pub fn watch(id: String) {
    let (tx, rx) = mpsc::channel();

    let rx_mutex = Arc::new(Mutex::new(rx));

    // show loading spinner instantly on click
    VPLAYER.with(|vplayer| if let Some((ref stack, ref overlay)) = *vplayer.borrow() {
        for child in overlay.get_children() {
            if WidgetExt::get_name(&child) == Some("GtkSpinner".into()) {
                let spinner: gtk::Spinner = child.clone().downcast().unwrap();
                spinner.start();
                spinner.show();
            } else {
                println!("{:?}", &child);
                overlay.remove(&child);
            }
        }
        stack.set_visible_child_name("page1");
    });

    thread::spawn(move || {
        let video_info = youtube::video_info(id);
        if tx.send(video_info).is_ok() {
            glib::idle_add(move || {
                VPLAYER.with(|vplayer| if let Some((ref stack, ref overlay)) = *vplayer.borrow() {
                    let rx = rx_mutex.lock().unwrap();
                    if let Ok(video_info) = rx.try_recv() {
                        match video_info {
                            Some(video_info) => {
                                for child in overlay.get_children() {
                                    if WidgetExt::get_name(&child) == Some("GtkSpinner".to_string()) {
                                        let spinner: gtk::Spinner = child.clone().downcast().unwrap();
                                        spinner.stop();
                                        spinner.hide();
                                    }
                                }
                                let related_videos = video_info.1;
                                let video_info = video_info.0;
                                let video_player = video_view(
                                    stack,
                                    &video_info,
                                    related_videos
                                );
                                overlay.add(&video_player);
                            }
                            None => {
                                for child in overlay.get_children() {
                                    if WidgetExt::get_name(&child) == Some("GtkSpinner".to_string()) {
                                        let spinner: gtk::Spinner = child.clone().downcast().unwrap();
                                        spinner.stop();
                                        spinner.hide();
                                    }
                                    let video_loading_error = video_loading_error("Could not fetch video.", stack);
                                    overlay.add(&video_loading_error);
                                }
                            }
                        };
                    }
                });
                glib::Continue(false)
            });
        }
    });
}

fn video_loading_error(error: &str, stack: &gtk::Stack) -> gtk::Grid {
    let builder = gtk::Builder::new_from_resource("/com/github/kil0meters/sentinel/gtk/video_player.ui");

    let error_view: gtk::Grid = builder.get_object("error_view").unwrap();
    let error_label: gtk::Label = builder.get_object("error_message").unwrap();
    let close_button: gtk::Button = builder.get_object("close_button1").unwrap();

    let error_message = format!(
        "<span size=\"x-large\">{}</span>",
        error
    );

    error_label.set_markup(&error_message);

    close_button.connect_clicked(clone!(stack => move |_| {
        stack.set_visible_child_name("page0");
    }));

    error_view
}

fn video_view(
    stack: &gtk::Stack,
    video_info: &youtube::Video,
    related_videos: Vec<youtube::Video>,
) -> (gtk::Grid) {
    let builder = gtk::Builder::new_from_resource("/com/github/kil0meters/sentinel/gtk/video_player.ui");

    let video_player_view: gtk::Grid = builder.get_object("video_player_view").unwrap();
    let close_button: gtk::Button = builder.get_object("close_button").unwrap();
    //let download_button: gtk::Button = builder.get_object("download_button").unwrap();
    //let share_button: gtk::Button = builder.get_object("share_button").unwrap();
    let related_videos_listbox: gtk::ListBox =
        builder.get_object("related_videos_listbox").unwrap();

    let video_title: gtk::Label = builder.get_object("video_title").unwrap();
    let video_author: gtk::Label = builder.get_object("video_author").unwrap();
    let video_views: gtk::Label = builder.get_object("video_views").unwrap();
    let video_description: gtk::Label = builder.get_object("video_description").unwrap();
    let video_likes: gtk::Label = builder.get_object("video_likes").unwrap();
    let video_dislikes: gtk::Label = builder.get_object("video_dislikes").unwrap();

    let title_markup = format!(
        "<span weight=\"bold\">{}</span>",
        encode_minimal(&video_info.title)
    );
    let author_markup = encode_minimal(&video_info.author);
    let description_markup = encode_minimal(&video_info.description);
    let views_markup = format!(
        "{} views",
        pretty_number(video_info.views.parse::<f64>().unwrap()),
    );
    let likes_markup = format!(
        "👍 {}",
        pretty_number(video_info.likes.parse::<f64>().unwrap())
    );
    let dislikes_markup = format!(
        "👎 {}",
        pretty_number(video_info.dislikes.parse::<f64>().unwrap())
    );

    video_title.set_markup(&title_markup);
    video_author.set_markup(&author_markup);
    video_views.set_markup(&views_markup);
    video_description.set_markup(&description_markup);
    video_likes.set_markup(&likes_markup);
    video_dislikes.set_markup(&dislikes_markup);

    related_videos_listbox.override_background_color(
        gtk::StateFlags::empty(),
        &gdk::RGBA {
            red: 0f64,
            green: 0f64,
            blue: 1f64,
            alpha: 0f64,
        },
    );
    // https://github.com/gtk-rs/gtk/issues/520
    related_videos_listbox.connect_row_selected(move |_, row| if let Some(row) = row.clone() {
        row.activate();
    });

    let mut thumbnails = vec![];
    let mut ids = vec![];
    for v in related_videos {
        let (v_widget, v_thumbnail) = video::new(&v.title, &v.author, &v.views, &v.duration);
        let v_listboxrow = gtk::ListBoxRow::new();
        v_listboxrow.add(&v_widget);
        v_listboxrow.show();
        related_videos_listbox.insert(&v_listboxrow, -1);
        let id = v.id.clone();

        v_listboxrow.connect_activate(move |_| {
            //download_button.connect_clicked(move |_| {
            let id = id.to_owned();
            watch(id);
        });
        thumbnails.push(v_thumbnail);
        ids.push(v.id);
    }
    load_thumbnails(thumbnails, ids);

    let stack_clone = stack.clone();
    close_button.connect_clicked(move |_| { stack_clone.set_visible_child_name("page0"); });

    (video_player_view)
}
