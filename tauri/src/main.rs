// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use raw_window_handle::HasRawWindowHandle;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::Manager;

            let window = app.get_window("entry").unwrap();
            window_shadows::set_shadow(&window, true).expect("Unsupported platform!");

            match window.raw_window_handle() {
                raw_window_handle::RawWindowHandle::Win32(handle) => {
                    println!("HWND: 0x{:08x}", handle.hwnd as usize);
                }
                _ => {
                    println!("Not a Win32 window");
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
