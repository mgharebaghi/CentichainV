use std::{fs::File, io::Write, path::Path};

use libp2p::futures::StreamExt;
use tauri::Emitter;

pub struct Downloader;

impl Downloader {
    pub async fn download<'a>(
        url: &str,
        file_name: &str,
        window: &tauri::Window,
    ) -> Result<(), &'a str> {
        let path = Path::new(file_name);
        if path.exists() {
            match std::fs::remove_file(file_name) {
                Ok(_) => Self::download_file(url, file_name, window).await,
                Err(_) => Err("Failed to remove zip file"),
            } // remove zip file if there is at first
        } else {
            Self::download_file(url, file_name, window).await
        }
    }

    async fn download_file<'a>(
        url: &str,
        file_name: &str,
        window: &tauri::Window,
    ) -> Result<(), &'a str> {
        let mut output = File::create(file_name).unwrap();
        let client = reqwest::Client::new();
        match client.get(url).send().await {
            Ok(response) => {
                let file_size = response.content_length().unwrap();
                let mut file_body = response.bytes_stream();
                let mut i = 0 as f64;

                loop {
                    match file_body.next().await {
                        Some(item) => {
                            let chunk = item.unwrap();
                            output.write_all(&chunk).unwrap();
                            i += chunk.len() as f64;
                            let percent = i / (file_size as f64) * 100.0;
                            window.emit("downloading", percent.round()).unwrap();
                        }
                        None => break,
                    }
                }
                output.flush().unwrap();
                Ok(window.emit("status", "Blockchain Dowanloaded").unwrap())
            }
            Err(_e) => Err("Request Problem For Download-(tools/downloader-46)"),
        }
    }
}
