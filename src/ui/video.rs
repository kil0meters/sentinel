use gtk;
use pango;

use htmlescape::encode_minimal;

#[derive(Debug)]
pub struct VideoWidgets {
    pub video: gtk::ListBoxRow,
    pub title: gtk::Label,
    pub spinner: gtk::Spinner,
    pub author: gtk::Label,
    pub views: gtk::Label,
    pub thumbnail: gtk::Image,
}

pub fn create_new_wide(
    title_string: &str,
    author_string: &str,
    views_string: &str,
) -> VideoWidgets {
    let video_builder = gtk::Builder::new_from_string(include_str!("../../data/ui/video.ui"));

    let video: gtk::ListBoxRow = video_builder.get_object("wide_video").unwrap();
    let title: gtk::Label = video_builder.get_object("wide_title").unwrap();
    let author: gtk::Label = video_builder.get_object("wide_author").unwrap();
    let views: gtk::Label = video_builder.get_object("wide_views").unwrap();
    let spinner: gtk::Spinner = video_builder.get_object("wide_spinner").unwrap();
    let thumbnail: gtk::Image = video_builder.get_object("wide_thumbnail").unwrap();

    let title_markup = format!(
        "<span weight=\"bold\">{}</span>",
        encode_minimal(title_string)
    );

    title.set_markup(&title_markup);
    author.set_markup(&encode_minimal(author_string));
    views.set_markup(views_string);

    title.set_ellipsize(pango::EllipsizeMode::End);
    title.set_xalign(0.0);

    VideoWidgets {
        video,
        title,
        spinner,
        author,
        views,
        thumbnail,
    }
}
