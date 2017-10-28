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

pub mod youtube {
    use std::io::Read;
    use reqwest;

    use select::document::Document;
    use select::predicate::{Class, Name, Predicate};

    use regex::Regex;

    #[derive(Debug, Clone)]
    pub struct Video {
        pub views: String,
        pub likes: String,
        pub dislikes: String,
        pub duration: String,
        pub title: String,
        pub author: String,
        pub id: String,
        pub thumbnail: Vec<u8>,
    }

    fn download_webpage(url: &str) -> Option<String> {
        let mut res = match reqwest::get(url) {
            Ok(ok) => ok,
            Err(_) => return None,
        };

        if res.status().is_success() {
            let mut content = String::new();
            res.read_to_string(&mut content).unwrap();
            return Some(content);
        }
        None
    }

    /*pub fn get_thumbnail(id: &str) -> Vec<u8> {
        let mut url = "https://i.ytimg.com/vi/".to_string();
        url.push_str(id);
        url.push_str("/mqdefault.jpg");

        let mut res = reqwest::get(&url).unwrap();

        println!("{}", &url);

        assert!(res.status().is_success());

        let mut image: Vec<u8> = vec![];

        res.read_to_end(&mut image).unwrap();

        image
    }*/

    /* pub fn video_info(id: String) /* -> Video */ {
        if id.len() > 10 || id.len() < 10 {
            panic!("Invalid Video ID")
        }

        let mut url = String::from("https://www.youtube.com/watch?v=");
        url.push_str(&id);
        let video_content = download_webpage(&url);
        let document = Document::from(video_content.as_str());

        // return video
    } */

    pub fn get_trending_videos() -> Option<Vec<Video>> {
        let trending_content = match download_webpage("https://www.youtube.com/feed/trending") {
            Some(x) => x,
            None => return None,
        };

        let document = Document::from(trending_content.as_str());

        let mut videos: Vec<Video> = Vec::new();
        let re = Regex::new(r"([a-zA-Z-.\s]|:+\s)").unwrap();

        for node in document.find(Class("yt-uix-tile").child(Class("yt-lockup-content"))) {
            let video_url_node = node.find(Class("yt-uix-tile-link")).next().unwrap();
            let video_url: Vec<_> = video_url_node.attr("href").unwrap().split('=').collect();

            let title = video_url_node.text();
            let author = node.find(Class("yt-lockup-byline").child(Class("yt-uix-sessionlink")))
                .next()
                .unwrap()
                .text();
            let id = video_url[1].to_string();

            let yt_lockup_meta_info_children: Vec<_> = node.find(
                Class("yt-lockup-meta-info").child(Name("li")),
            ).collect();

            let views = if yt_lockup_meta_info_children.get(1).is_none() {
                "hidden".to_string()
            } else {
                yt_lockup_meta_info_children[1].text()
            };

            let duration = if node.find(Class("accessible-description")).next().is_none() {
                "LIVE".to_string()
            } else {
                let duration_raw = node.find(Class("accessible-description")).next().unwrap();
                re.replace_all(&duration_raw.text(), "").to_string()
            };

            let likes = "".to_string();
            let dislikes = "".to_string();

            //let mut thumbnail_url = "https://i.ytimg.com/vi/".to_string();
            //thumbnail_url.push_str(&id);
            //thumbnail_url.push_str("mqdefault.jpg");

            let thumbnail = vec![]; // get_thumbnail(&thumbnail_url);

            let video = Video {
                views,
                likes,
                dislikes,
                duration,
                title,
                author,
                id,
                thumbnail,
            };
            videos.push(video);
        }
        Some(videos)
    }
}
