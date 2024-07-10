// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem,Manager};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit 关闭");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide 隐藏");

    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| menu_handle(app, event))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


fn toggle_window_visibility(app_handle: &tauri::AppHandle, item_handle: &tauri::SystemTrayMenuItemHandle) {
    let window = app_handle.get_window("main").unwrap(); // 确保这里的标签与 tauri.conf.json 中一致
    if window.is_visible().unwrap() {
        window.hide().unwrap();
        item_handle.set_title("显示窗口").unwrap();
    } else {
        window.show().unwrap();
        window.set_focus().unwrap(); // 将窗口设置为活动窗口并获得焦点
        item_handle.set_title("隐藏窗口").unwrap();
    }
}

fn menu_handle(app_handle: &tauri::AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("鼠标-左击");
            let item_handle = app_handle.tray_handle().get_item("hide");
            toggle_window_visibility(app_handle, &item_handle);
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("鼠标-右击");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("鼠标-双击");
            let item_handle = app_handle.tray_handle().get_item("hide");
            toggle_window_visibility(app_handle, &item_handle);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let item_handle = app_handle.tray_handle().get_item(&id);
                toggle_window_visibility(app_handle, &item_handle);
            }
            _ => {}
        },
        _ => {}
    }
}
