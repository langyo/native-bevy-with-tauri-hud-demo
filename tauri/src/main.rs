#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use raw_window_handle::HasRawWindowHandle;
use windows::Win32::{
    Foundation::{HWND, POINT, RECT},
    Graphics::Gdi::ClientToScreen,
    UI::WindowsAndMessaging::{GetClientRect, SetWindowPos, HWND_TOPMOST, SWP_NOACTIVATE},
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

                        loop {
                            unsafe {
                                let mut rect: RECT = Default::default();
                                GetClientRect(bevy_hwnd.clone(), &mut rect).unwrap();

                                let mut left_top: POINT = POINT {
                                    x: rect.left,
                                    y: rect.top,
                                };
                                let mut right_bottom: POINT = POINT {
                                    x: rect.right,
                                    y: rect.bottom,
                                };
                                ClientToScreen(bevy_hwnd, &mut left_top);
                                ClientToScreen(bevy_hwnd, &mut right_bottom);

                                let width = rect.right - rect.left;
                                let height = rect.bottom - rect.top;

                                SetWindowPos(
                                    tauri_hwnd,
                                    HWND_TOPMOST,
                                    left_top.x,
                                    left_top.y,
                                    width,
                                    height,
                                    SWP_NOACTIVATE,
                                )
                                .unwrap();
                            }
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
