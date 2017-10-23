use gtk;
use pango;

use htmlescape::encode_minimal;

#[derive(Debug)]
pub struct VideoWidgets {
    pub video: gtk::ListBoxRow,
    pub title: gtk::Label,
    pub author: gtk::Label,
    pub views: gtk::Label,
    pub thumbnail: gtk::Image,
}

impl VideoWidgets {
    fn new(video: gtk::ListBoxRow, title: gtk::Label, author: gtk::Label,
           views: gtk::Label, thumbnail: gtk::Image) -> VideoWidgets {
        return VideoWidgets { video, title, author, views, thumbnail };
    }
}

pub fn create_new_wide(title_string: &str, author_string: &str, views_string: &str) -> VideoWidgets {
    let video_builder = gtk::Builder::new_from_string(include_str!("../../data/ui/video.ui"));

    let video: gtk::ListBoxRow = video_builder.get_object("wide_video").unwrap();
    let title: gtk::Label = video_builder.get_object("wide_title").unwrap();
    let author: gtk::Label = video_builder.get_object("wide_author").unwrap();
    let views: gtk::Label = video_builder.get_object("wide_views").unwrap();
    let thumbnail: gtk::Image = video_builder.get_object("wide_thumbnail").unwrap();

    let mut title_markup = "<span weight=\"bold\">".to_string();
    title_markup.push_str(&encode_minimal(title_string));
    title_markup.push_str("</span>");

    title.set_markup(&title_markup);
    author.set_markup(&encode_minimal(author_string));
    views.set_markup(views_string);

    title.set_ellipsize(pango::EllipsizeMode::End);
    title.set_xalign(0.0);

    let displayed_video = VideoWidgets::new(video, title, author, views, thumbnail);
    return displayed_video;
}
