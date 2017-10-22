use gtk;

use ui::video;
#[allow(unused_imports)]
use lib::youtube;
/*
pub fn initialize_trending_videos(number: usize) -> (Vec<video::VideoWidgets>, gtk::ListBox) {
    let mut trending_videos: Vec<video::VideoWidgets> = vec![];
    let list_box = gtk::ListBox::new();
    for i in 0..number {
        let trending_video = video::create_new_wide();
        trending_videos.push(trending_video);
        list_box.insert(&trending_videos[i].video, -1);
    }
    return (trending_videos, list_box);
}

pub fn load_new_trending_videos(number: usize) {
    println!("{} videos loaded", number);
}

macro_rules! update_trending{
    ($tx:ident) => {{
        let tx_clone = $tx.clone();
        thread::spawn(move || {
            let video_data = youtube::get_trending_videos();
            tx_clone.send(video_data).unwrap();
            glib::idle_add(load_trending);
        });
    }}
}*/
