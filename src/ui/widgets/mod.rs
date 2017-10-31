use gtk;

pub mod video_wide;

pub struct VideoWidget {
    pub video: gtk::ListBoxRow,
    pub title: gtk::Label,
    pub author: gtk::Label,
    pub views: gtk::Label,
    pub duration: gtk::Label,
    pub thumbnail: gtk::Image,
}
