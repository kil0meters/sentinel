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
use gdk;
use gtk::prelude::*;

use lib::youtube;

use htmlescape::encode_minimal;
use lib::utils::pretty_number;
use ui::widgets::video;
use ui::utils::load_thumbnails;
use ui::video_player;

pub fn new(
    stack: &gtk::Stack,
    video_info: youtube::Video,
    related_videos: Vec<youtube::Video>,
) -> (gtk::Grid) {
    let builder = include_str!("../../../data/ui/video_player.ui");
    let builder = gtk::Builder::new_from_string(builder);

    let video_player_view: gtk::Grid = builder.get_object("video_player_view").unwrap();
    let close_button: gtk::Button = builder.get_object("close_button").unwrap();
    let download_button: gtk::Button = builder.get_object("download_button").unwrap();
    let share_button: gtk::Button = builder.get_object("share_button").unwrap();
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
    let views_markup = pretty_number(video_info.views.parse::<f64>().unwrap());
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
    related_videos_listbox.connect_row_selected(move |_, row| {
        println!("hi");
        if let Some(row) = row.clone() {
            row.activate();
        }
        println!("hi");
    });

    let mut thumbnails = vec![];
    let mut ids = vec![];
    for v in related_videos {
        let (v_widget, v_thumbnail) = video::new(&v.title, &v.author, &v.views, &v.duration, &v.id);
        let v_listboxrow = gtk::ListBoxRow::new();
        v_listboxrow.add(&v_widget);
        v_listboxrow.show();
        related_videos_listbox.insert(&v_listboxrow, -1);
        let id = v.id.clone();

        v_listboxrow.connect_activate(move |_| {
        //download_button.connect_clicked(move |_| {
        let id = id.to_owned();
            video_player::watch(id);
        });
        thumbnails.push(v_thumbnail);
        ids.push(v.id);
    }
    load_thumbnails(thumbnails, ids);

    let stack_clone = stack.clone();
    close_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("page0");
    });

    (video_player_view)
}
