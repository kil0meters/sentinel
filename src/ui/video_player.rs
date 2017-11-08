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
use glib;
use gtk::prelude::*;

use ui::VPLAYER_STACK;
use lib::youtube;

use std::thread;
use std::sync::{mpsc, Arc, Mutex};

use ui::widgets::video_player_view;

// this is ugly.

macro_rules! build_player {
    ($rx_mutex:ident) => {{
    move || {
        VPLAYER_STACK.with(|vplayer| {
            let rx = $rx_mutex.lock().unwrap();
            if let Some((ref stack, ref overlay)) = *vplayer.borrow() {
                if let Ok(video_info) = rx.try_recv() {
                    match video_info {
                        Some(video_info) => {
                            for child in overlay.get_children() {
                                if child.get_name() == Some("GtkSpinner".to_string()) {
                                    let spinner: gtk::Spinner = child.clone().downcast().unwrap();
                                    spinner.stop();
                                    spinner.hide();
                                }
                            }
                            let related_videos = video_info.1;
                            let video_info = video_info.0;
                            let video_player = video_player_view::new(
                                &stack,
                                video_info,
                                related_videos
                            );
                            overlay.add(&video_player);
                        }
                        None => {}
                    }
                }
            }
        });
        glib::Continue(false)
    }}}
}

pub fn watch(id: String) {
    let (tx, rx) = mpsc::channel();

    let rx_mutex = Arc::new(Mutex::new(rx));

    // show loading spinner instantly on click
    VPLAYER_STACK.with(|vplayer| {
        if let Some((ref stack, ref overlay)) = *vplayer.borrow() {
            for child in overlay.get_children() {
                if child.get_name() == Some("GtkSpinner".into()) {
                    let spinner: gtk::Spinner = child.clone().downcast().unwrap();
                    spinner.start();
                    spinner.show();
                } else {
                    child.destroy();
                }
            }
            stack.set_visible_child_name("page1");
        }
    });

    thread::spawn(move || {
        let video_info = youtube::video_info(id);
        if tx.send(video_info).is_ok() {
            glib::idle_add(build_player!(rx_mutex));
        }
    });
}
