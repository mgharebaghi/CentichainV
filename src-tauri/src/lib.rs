mod events;
mod generator;
mod prestart;
mod tools;
use events::start;
use prestart::{
    keys::{check_key, generate_keys},
    memory_check,
};
use tauri::Emitter;
use tauri_plugin_updater::UpdaterExt;
use tools::{
    exit::exit,
    for_front::{
        blocks::latest_blocks, centies::sum_centies, make_trx::send_transaction,
        mongodb::mongodb_download,
    },
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            memory_check,
            check_key,
            start,
            exit,
            sum_centies,
            latest_blocks,
            send_transaction,
            mongodb_download,
            generate_keys,
            check_for_updates,
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(handle).await.unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn update(app: tauri::AppHandle) -> tauri::Result<()> {
    if let Some(update) = app.updater().unwrap().check().await.unwrap() {
        let mut downloaded = 0;

        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await
            .unwrap();

        println!("update installed");
        app.restart();
    }

    Ok(())
}

#[tauri::command]
async fn check_for_updates(app: tauri::AppHandle, window: tauri::Window) -> Result<(), String> {
    match app.updater().unwrap().check().await {
        Ok(Some(update)) => {
            window.emit("available", true).map_err(|e| e.to_string())?;
            let mut downloaded = 0;

            match update
                .download_and_install(
                    |chunk_length, content_length| {
                        downloaded += chunk_length;
                        window.emit("progress", content_length).unwrap();
                    },
                    || {
                        window.emit("status", "Download finished").unwrap();
                    },
                )
                .await
            {
                Ok(_) => {
                    window
                        .emit("status", "Update installed")
                        .map_err(|e| e.to_string())?;
                    app.restart()
                }
                Err(e) => Err(format!("Failed to download and install update: {}", e)),
            }
        }
        Ok(None) => {
            window.emit("available", false).map_err(|e| e.to_string())?;
            Ok(())
        }
        Err(e) => Err(format!("Failed to check for updates: {}", e)),
    }
}
