// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use raw_window_handle::HasRawWindowHandle;
use windows::Win32::{
    Foundation::{COLORREF, HWND},
    UI::WindowsAndMessaging::{
        SetLayeredWindowAttributes, SetParent, SetWindowLongPtrA, SetWindowPos, GWL_EXSTYLE,
        GWL_STYLE, LWA_ALPHA, SWP_NOSIZE, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TOPMOST,
        WS_EX_TRANSPARENT, WS_POPUP, WS_VISIBLE,
    },
};

fn main() {
    let bevy_hwnd = std::env::args().nth(1);
    let bevy_hwnd: isize = bevy_hwnd.unwrap().parse().unwrap();

    tauri::Builder::default()
        .setup(move |app| {
            use tauri::Manager;

            let window = app.get_window("entry").unwrap();
            match window.raw_window_handle() {
                raw_window_handle::RawWindowHandle::Win32(handle) => {
                    let tauri_hwnd = handle.hwnd as isize;
                    println!("HWND: 0x{:08x}", tauri_hwnd);

                    std::thread::spawn(move || {
                        let tauri_hwnd = HWND(tauri_hwnd as _);
                        let bevy_hwnd = HWND(bevy_hwnd as _);

                        unsafe {
                            SetWindowPos(tauri_hwnd, bevy_hwnd, 0, 0, 0, 0, SWP_NOSIZE).unwrap();
                            SetParent(tauri_hwnd, bevy_hwnd);

                            let style = WS_POPUP.0 | WS_VISIBLE.0;
                            SetWindowLongPtrA(tauri_hwnd, GWL_STYLE, style as isize);
                            let style = WS_EX_LAYERED.0
                                | WS_EX_NOACTIVATE.0
                                | WS_EX_TOPMOST.0
                                | WS_EX_TRANSPARENT.0;
                            SetWindowLongPtrA(tauri_hwnd, GWL_EXSTYLE, style as isize);

                            // TODO - 虽然嵌入成功了，但是透明度设置失败了
                            SetLayeredWindowAttributes(tauri_hwnd, COLORREF(0), 255, LWA_ALPHA)
                                .unwrap();
                        }
                    });
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
