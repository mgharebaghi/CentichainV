use sysinfo::System;
pub mod keys;

//check memory size and if it was under 4 return error
#[tauri::command]
pub fn memory_check() -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    let memory_size = sys.total_memory() / 1024 / 1024 / 1024;
    if memory_size >= 4 {
        true
    } else {
        false
    }
}
