use gtk;
use gtk::prelude::*;

use htmlescape::encode_minimal;
use sentinel_api::utils::pretty_number;

pub fn new(
    title_string: &str,
    author_string: &str,
    views_string: &str,
    duration_string: &str,
) -> (gtk::Grid, gtk::Image) {
    let builder = gtk::Builder::new_from_resource("/com/github/kil0meters/sentinel/gtk/widgets.ui");

    let video: gtk::Grid = builder.get_object("video").unwrap();
    let title: gtk::Label = builder.get_object("video_title").unwrap();
    let author_and_views: gtk::Label = builder.get_object("video_author_and_views").unwrap();
    let duration: gtk::Label = builder.get_object("video_duration").unwrap();
    let thumbnail: gtk::Image = builder.get_object("video_thumbnail").unwrap();

    let title_markup = format!(
        "<span weight=\"semibold\" font=\"11\">{}</span>",
        encode_minimal(title_string)
    );
    let author_and_views_markup =
        format!(
        "{} · {} views",
        encode_minimal(author_string),
        pretty_number(views_string.parse::<f64>().unwrap()),
    );

    title.set_markup(&title_markup);
    author_and_views.set_markup(&author_and_views_markup);
    duration.set_markup(duration_string);

    (video, thumbnail)
}
