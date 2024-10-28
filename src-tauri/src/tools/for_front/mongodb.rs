use std::{
    env::consts::OS,
    fs::{self, File},
    io::Write,
    os::windows::process::CommandExt,
    path::Path,
    process::Command,
    thread,
};

use libp2p::futures::StreamExt;
use tauri::Emitter;

#[tauri::command]
pub async fn mongodb_download(window: tauri::Window) {
    if OS == "windows" {
        let path = Path::new("mongodb.msi");
        let mongo_path = Path::new("c:\\Program Files\\MongoDB\\Server\\8.0\\bin");

        if mongo_path.exists() {
            window.emit("mongodb", "installed".to_string()).unwrap();
        } else {
            if path.exists() {
                // Remove existing MongoDB installer if present
                fs::remove_file(path).expect("Failed to remove existing file");
            }
            let url = "https://fastdl.mongodb.org/windows/mongodb-windows-x86_64-8.0.0-signed.msi";
            let save_file_path = "mongodb.msi";
            let resp = reqwest::get(url).await.expect("request failed");
            let size = resp.content_length().unwrap();
            let mut body = resp.bytes_stream();
            let mut out = fs::File::create(save_file_path).expect("file create error");
            let mut i = 0 as f64;
            loop {
                match body.next().await {
                    Some(item) => {
                        let chunk = item.unwrap();
                        out.write_all(&chunk).unwrap();
                        i += chunk.len() as f64;
                        let percent = i / (size as f64) * 100.0;
                        // Update download progress
                        window.emit("DlPercent", percent.round()).unwrap();
                    }
                    None => break,
                }
            }

            // Notify that MongoDB has been downloaded
            window.emit("mongodb", "downloaded".to_string()).unwrap();
            out.sync_all().unwrap();

            match create_install_script() {
                Ok(_) => {
                    // Run the installation script
                    match Command::new("cmd")
                        .args(&["/C", "install_mongodb.bat"])
                        .creation_flags(0x08000000) // CREATE_NO_WINDOW flag
                        .spawn()
                    {
                        Ok(mut child) => {
                            // Spawn a new thread to wait for the installation to complete
                            let window_clone = window.clone();
                            thread::spawn(move || {
                                // Wait for the installation to complete
                                match child.wait() {
                                    Ok(status) => {
                                        if status.success() {
                                            // Check if MongoDB is now installed
                                            if Path::new(
                                                "c:\\Program Files\\MongoDB\\Server\\8.0\\bin",
                                            )
                                            .exists()
                                            {
                                                window_clone
                                                    .emit("mongodb", "installed".to_string())
                                                    .unwrap();
                                            } else {
                                                window_clone.emit("mongodb", "Installation completed but MongoDB folder not found".to_string()).unwrap();
                                            }
                                        } else {
                                            window_clone
                                                .emit(
                                                    "mongodb",
                                                    format!(
                                                        "Installation failed with status: {}",
                                                        status
                                                    ),
                                                )
                                                .unwrap();
                                        }
                                    }
                                    Err(e) => {
                                        window_clone
                                            .emit(
                                                "mongodb",
                                                format!("Failed to wait for installation: {}", e),
                                            )
                                            .unwrap();
                                    }
                                }
                            });

                            // Continue with the rest of the function without waiting
                            window.emit("mongodb", "downloaded".to_string()).unwrap();
                        }
                        Err(e) => {
                            window
                                .emit("mongodb", format!("Failed to start installation: {}", e))
                                .unwrap();
                        }
                    }
                }
                Err(e) => {
                    window
                        .emit(
                            "mongodb",
                            format!("Failed to create installation script: {}", e),
                        )
                        .unwrap();
                }
            }
        }
    }
}

fn create_install_script() -> std::io::Result<()> {
    let script_content = r#"
@echo off
msiexec /l*v mdbinstall.log /qb /i "%~dp0mongodb.msi" ADDLOCAL=ServerService SHOULD_INSTALL_COMPASS=0
if %ERRORLEVEL% EQU 0 (
    echo MongoDB installation completed successfully > mongodb_install_result.txt
) else (
    echo MongoDB installation failed with error code %ERRORLEVEL% > mongodb_install_result.txt
    type mdbinstall.log >> mongodb_install_result.txt
)
"#;

    let mut file = File::create("install_mongodb.bat")?;
    file.write_all(script_content.as_bytes())?;
    Ok(())
}
