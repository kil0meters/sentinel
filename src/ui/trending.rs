macro_rules! initialize_trending {
    ($trending_spinner:ident, $trending_viewport:ident) => {{
        let (tx, rx) = channel();

        thread_local! (
            static LOADING: RefCell<Option<(gtk::Spinner, gtk::Viewport, Receiver<Vec<youtube::Video>>)>> = RefCell::new(None);
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
                    if let Some((ref trending_spinner, ref trending_viewport, ref rx)) = *loading.borrow() {
                        if let Ok(trending_videos) = rx.try_recv() {
                            let trending_builder = gtk::Builder::new_from_string(include_str!("../../data/ui/trending_view.ui"));
                            let trending_listbox: gtk::ListBox = trending_builder.get_object("trending_listbox").unwrap();

                            for i in 0..trending_videos.len() {
                                let video_widget = video::create_new_wide(&trending_videos[i].title,
                                                                          &trending_videos[i].author,
                                                                          &trending_videos[i].views);

                                trending_listbox.insert(&video_widget.video, -1);
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
