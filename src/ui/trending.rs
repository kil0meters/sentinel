// *--------------------------------------*
// | Beware!                              |
// | Giant jumble of spagetti code ahead. |
// *--------------------------------------*
//         \   ^__^
//          \  (oo)\_______
//             (__)\       )\/\
//                 ||----w |
//                 ||     ||

macro_rules! initialize_trending {
    ($trending_spinner:ident, $trending_viewport:ident) => {{
        use std::sync::mpsc::{channel, Receiver};
        use std::cell::RefCell;
        use std::thread;

        let (tx, rx) = channel();

        thread_local! (
            static LOADING: RefCell<Option<(
                gtk::Spinner,
                gtk::Viewport,
                Receiver<Option<Vec<youtube::Video>>>
            )>> = RefCell::new(None);
        );

        LOADING.with(move |loading| {
            *loading.borrow_mut() = Some(($trending_spinner, $trending_viewport, rx));
        });

        thread::spawn(move || {
            let trending_videos = youtube::get_trending_videos();
            tx.send(trending_videos)
                .expect("couldn't send data to thread");
            glib::idle_add(move || {
                LOADING.with(|loading| {
                    if let Some((
                            ref trending_spinner,
                            ref trending_viewport,
                            ref rx
                        )) = *loading.borrow() {
                        if let Ok(trending_videos) = rx.try_recv() {

                            // I should probably fix this in the future.
                            // Although, the error handling is done
                            // in ui/mod.rs so it shouldn't cause issues.
                            let trending_videos = trending_videos.unwrap();

                            let trending_builder = gtk::Builder::new_from_string(
                                include_str!("../../data/ui/trending_view.ui")
                            );
                            let trending_listbox: gtk::ListBox = trending_builder.get_object(
                                "trending_listbox"
                            ).unwrap();

                            let mut video_widgets: Vec<video::VideoWidgets> = vec![];
                            for video in &trending_videos {
                                let video_widget = video::create_new_wide(
                                    &video.title,
                                    &video.author,
                                    &video.views
                                );
                                trending_listbox.insert(&video_widget.video, -1);
                                video_widgets.push(video_widget);
                            }

                            trending_listbox.show_all();
                            trending_spinner.destroy();
                            trending_viewport.add(&trending_listbox);
                        }
                    }
                });
                glib::Continue(false)
            });
        });
    }}
}
