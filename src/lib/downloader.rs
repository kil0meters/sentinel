use std::fs::{self, DirBuilder, File};
use std::io::{BufWriter, Read, Write};
use reqwest;
use hyper::header::*;

/// Downloads `url` to `target`
pub fn download_to(dir: &str, file: &str, url: &str) {
    let mut res = reqwest::get(url).unwrap();

    if res.status().is_success() {
        DirBuilder::new().recursive(true).create(dir).unwrap();

        let headers = res.headers().clone();

        let content_length = headers
            .get::<ContentLength>()
            .map(|content_length| **content_length);

        let chunk_size = match content_length {
            Some(x) => x as u8 / 99,
            None => 128 as u8, // default
        };

        let out_file = format!("{}/{}.part", dir, file);
        let mut writer = BufWriter::new(File::create(&out_file).unwrap());

        loop {
            let mut buffer = vec![0, chunk_size];
            let buffer_count = res.read(&mut buffer[..]).unwrap();
            buffer.truncate(buffer_count);
            if !buffer.is_empty() {
                writer.write_all(buffer.as_slice()).unwrap();
            } else {
                break;
            }
        }
        let target = format!("{}/{}", dir, file);
        fs::rename(out_file, target).unwrap();
    }
}
