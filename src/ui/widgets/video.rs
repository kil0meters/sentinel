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
use gtk::prelude::*;

use htmlescape::encode_minimal;
use lib::utils::pretty_number;

pub fn new(
    title_string: &str,
    author_string: &str,
    views_string: &str,
    duration_string: &str,
    id: &str,
) -> (gtk::Grid, gtk::Image) {
    let builder = include_str!("../../../data/ui/widgets.ui");
    let builder = gtk::Builder::new_from_string(builder);

    let video: gtk::Grid = builder.get_object("video").unwrap();
    let title: gtk::Label = builder.get_object("video_title").unwrap();
    let author_and_views: gtk::Label = builder.get_object("video_author_and_views").unwrap();
    let duration: gtk::Label = builder.get_object("video_duration").unwrap();
    let thumbnail: gtk::Image = builder.get_object("video_thumbnail").unwrap();

    let title_markup = format!(
        "<span weight=\"semibold\" font=\"11\">{}</span>",
        encode_minimal(title_string)
    );
    let author_and_views_markup = format!(
        "{} · {}",
        encode_minimal(author_string),
        pretty_number(views_string.parse::<f64>().unwrap()),
    );

    title.set_markup(&title_markup);
    author_and_views.set_markup(&author_and_views_markup);
    duration.set_markup(duration_string);

    (video, thumbnail)
}
