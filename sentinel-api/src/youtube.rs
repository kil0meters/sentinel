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

use std::io::Read;
use reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
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
    pub description: String,
}

impl Video {
    #[allow(unknown_lints, too_many_arguments)]
    fn new(
        views: String,
        likes: String,
        dislikes: String,
        duration: String,
        title: String,
        author: String,
        id: String,
        description: String,
    ) -> Video {
        Video {
            views,
            likes,
            dislikes,
            duration,
            title,
            author,
            id,
            description,
        }
    }
}

fn download_webpage(url: &str) -> Option<String> {
    let mut res = match reqwest::get(url) {
        Ok(x) => x,
        Err(_) => return None,
    };

    if res.status().is_success() {
        let mut content = String::new();
        res.read_to_string(&mut content).unwrap();
        return Some(content);
    }
    None
}

pub fn video_info(id: String) -> Option<(Video, Vec<Video>)> {
    let url = format!("https://www.youtube.com/watch?v={}", id);
    let video_content = match download_webpage(&url) {
        Some(x) => x,
        None => return None,
    };

    let re = Regex::new(r"[^0-9]").unwrap();
    let title_regex = Regex::new(r"^[\s\xA0]+|[\s\xA0]+$").unwrap();
    let document = Document::from(video_content.as_str());

    let views_raw = document
        .find(Class("watch-view-count"))
        .next()
        .unwrap()
        .text();
    let likes_raw = document
        .find(Class("like-button-renderer-like-button"))
        .next()
        .unwrap()
        .text();
    let dislikes_raw = document
        .find(Class("like-button-renderer-dislike-button"))
        .next()
        .unwrap()
        .text();
    let title = document
        .find(Attr("id", "eow-title"))
        .next()
        .unwrap()
        .attr("title")
        .unwrap()
        .into();
    let author = document
        .find(Class("yt-user-info").child(Class("yt-uix-sessionlink")))
        .next()
        .unwrap()
        .text();
    let description_node = document.find(Attr("id", "eow-description")).next().unwrap();

    let views = re.replace_all(&views_raw, "").into();
    let likes = re.replace_all(&likes_raw, "").into();
    let dislikes = re.replace_all(&dislikes_raw, "").into();

    // count the br tag as a new line
    let mut description = String::new();
    for node in description_node.children() {
        if node.name() == Some("br") {
            description.push('\n');
        } else {
            description.push_str(&node.text());
        }
    }

    let mut related_videos = vec![];

    for node in document.find(Class("video-list-item")) {
        let r_views_raw = node.find(Class("view-count"))
            .next()
            .unwrap()
            .text()
            .to_string();
        let r_views = re.replace_all(&r_views_raw, "").into();
        let r_duration = node.find(Class("video-time")).next().unwrap().text();
        let r_title_raw = node.find(Class("title")).next().unwrap().text();
        let r_title = title_regex.replace_all(&r_title_raw, "").into();
        let r_author = node.find(Class("attribution").child(Name("span")))
            .next()
            .unwrap()
            .text();
        let r_id_raw: Vec<_> = node.find(Class("yt-uix-sessionlink"))
            .next()
            .unwrap()
            .attr("href")
            .unwrap()
            .split('=')
            .collect();
        let r_id = r_id_raw[1].into();

        related_videos.push(Video::new(
            r_views,
            "".into(),
            "".into(),
            r_duration,
            r_title,
            r_author,
            r_id,
            "".into(),
        ));
    }

    Some((
        Video::new(
            views,
            likes,
            dislikes,
            "".into(),
            title,
            author,
            id,
            description,
        ),
        related_videos,
    ))
}

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
        let id = video_url[1].into();

        let yt_lockup_meta_info_children: Vec<_> = node.find(Class("yt-lockup-meta-info").child(
            Name("li"),
        )).collect();

        let views = if yt_lockup_meta_info_children.get(1).is_none() {
            "hidden".into()
        } else {
            yt_lockup_meta_info_children[1].text()
        };

        let duration_node = node.find(Class("accessible-description")).next();
        let duration = if duration_node.is_none() {
            "LIVE".into()
        } else {
            let duration_raw = duration_node.unwrap().text();
            re.replace_all(&duration_raw, "").into()
        };

        let video = Video::new(
            views,
            "".into(),
            "".into(),
            duration,
            title,
            author,
            id,
            "".into(),
        );
        videos.push(video);
    }
    Some(videos)
}
