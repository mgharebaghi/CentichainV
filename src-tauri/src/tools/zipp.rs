use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use tauri::Emitter;

pub struct Zip;

impl Zip {
    pub fn extract<'a>(to: &str, window: &tauri::Window) -> Result<(), &'a str> {
        let path = Path::new("./etc");
        if path.exists() {
            match fs::remove_dir_all("./etc") {
                Ok(_) => Self::extract_zip(to, window),
                Err(_) => return Err("Failed to remove etc folder"),
            }
        } else {
            Self::extract_zip(to, window)
        }
    }

    fn extract_zip<'a>(to: &str, window: &tauri::Window) -> Result<(), &'a str> {
        fs::create_dir_all(to).unwrap();
        match zip::ZipArchive::new(File::open("Centichain.zip").unwrap()) {
            Ok(mut archive) => {
                for i in 0..archive.len() {
                    let mut item = archive.by_index(i).unwrap();
                    if item.is_file() {
                        let mut output = File::create(item.name()).unwrap();
                        let mut bytes = Vec::new();
                        item.read_to_end(&mut bytes).unwrap();
                        output.write_all(&bytes).unwrap();
                    }
                }
                Ok(window
                    .emit("status", "Zip File Of Blockchain Extracted")
                    .unwrap())
            }
            Err(_e) => Err("Extract Zip File Error-(event/syncing-29)"),
        }
    }
}
