use gtk;
use gtk::prelude::*;

use htmlescape::encode_minimal;

pub fn new(
    title_string: &str,
    author_string: &str,
    views_string: &str,
    duration_string: &str,
) -> (gtk::ListBoxRow, gtk::Image) {
    let video_builder = gtk::Builder::new_from_resource("/com/github/kil0meters/sentinel/gtk/widgets.ui");

    let video: gtk::ListBoxRow = video_builder.get_object("wide_video").unwrap();
    let title: gtk::Label = video_builder.get_object("wide_video_title").unwrap();
    let author: gtk::Label = video_builder.get_object("wide_video_author").unwrap();
    let views: gtk::Label = video_builder.get_object("wide_video_views").unwrap();
    let duration: gtk::Label = video_builder.get_object("wide_video_duration").unwrap();
    let thumbnail: gtk::Image = video_builder.get_object("wide_video_thumbnail").unwrap();

    let title_markup = format!(
        "<span weight=\"semibold\" font=\"11\">{}</span>",
        encode_minimal(title_string)
    );

    title.set_markup(&title_markup);
    author.set_markup(&encode_minimal(author_string));
    views.set_markup(views_string);
    duration.set_markup(duration_string);

    (video, thumbnail)
}
