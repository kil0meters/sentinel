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
use pango;

use htmlescape::encode_minimal;

use gtk::prelude::*;

pub fn new(title_string: &str, author_string: &str, views_string: &str) -> gtk::ListBoxRow {
    let video_builder = gtk::Builder::new_from_string(include_str!("../../../data/ui/video.ui"));

    let video: gtk::ListBoxRow = video_builder.get_object("wide_video").unwrap();
    let title: gtk::Label = video_builder.get_object("wide_title").unwrap();
    let author: gtk::Label = video_builder.get_object("wide_author").unwrap();
    let views: gtk::Label = video_builder.get_object("wide_views").unwrap();
    // let thumbnail: gtk::Image = video_builder.get_object("wide_thumbnail").unwrap();

    let title_markup = format!(
        "<span weight=\"bold\">{}</span>",
        encode_minimal(title_string)
    );

    title.set_markup(&title_markup);
    author.set_markup(&encode_minimal(author_string));
    views.set_markup(views_string);

    title.set_ellipsize(pango::EllipsizeMode::End);
    title.set_xalign(0.0);

    video
}
