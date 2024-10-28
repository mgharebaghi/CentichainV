#[tauri::command]
pub async fn exit() {
    std::process::exit(0)
}
