use gtk;
use pango;
use gtk::prelude::*;

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

pub fn create_new_wide() -> VideoWidgets {
    let video = gtk::ListBoxRow::new();
    let layout = gtk::Layout::new(None,None);

    let title = gtk::Label::new(None);
    let author = gtk::Label::new(None);
    let views = gtk::Label::new(None);
    let thumbnail = gtk::Image::new();

    title.set_markup("<span weight=\"bold\">### ####### ## #####</span>");
    author.set_markup("#######");
    views.set_markup("#,###,###,### #####");

    title.set_ellipsize(pango::EllipsizeMode::End);
    title.set_xalign(0.0);

    layout.set_size_request(720, 151);
    thumbnail.set_size_request(240, 135);
    title.set_size_request(456, 0);

    layout.add(&title);
    layout.add(&views);
    layout.add(&author);
    layout.add(&thumbnail);

    layout.set_child_x(&title, 256);
    layout.set_child_y(&title, 8);

    layout.set_child_x(&author, 256);
    layout.set_child_y(&author, 32);

    layout.set_child_x(&views, 256);
    layout.set_child_y(&views, 56);

    layout.set_child_x(&thumbnail, 8);
    layout.set_child_y(&thumbnail, 8);

    video.add(&layout);

    let displayed_video = VideoWidgets::new(video, title, author, views, thumbnail);
    return displayed_video;
}
