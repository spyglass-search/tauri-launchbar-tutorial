#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::{App, AppHandle, GlobalShortcutManager, Manager};

const SHORTCUT: &str = "Ctrl+Space";
const WINDOW: &str = "launcher";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(window: tauri::Window, name: &str) {
    println!("Input received: {}", name);
    toggle_launchbar(&window.app_handle());
}

fn toggle_launchbar(app: &AppHandle) {
    let window = app.get_window(WINDOW).expect("Did you label your window?");
    if let Ok(true) = window.is_visible() {
        let _ = window.hide();
    } else {
        let _ = window.show();
    }
}

fn register_shortcut(app: &mut App) -> Result<(), tauri::Error> {
    let app_handle = app.app_handle();
    let mut shortcuts = app_handle.global_shortcut_manager();
    if !shortcuts.is_registered(SHORTCUT)? {
        shortcuts.register(SHORTCUT, move || toggle_launchbar(&app_handle))?;
    }

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(move |app: &mut App| {
            if let Err(err) = register_shortcut(app) {
                eprintln!("Unable to register shortcut: {err}");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
