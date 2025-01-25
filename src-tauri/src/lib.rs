use libmediatool;
use std::path::Path;
use tauri::{AppHandle, Emitter};
//use tauri::Manager;
use std::thread;
use tauri::menu::MenuBuilder;

#[tauri::command]
fn get_media_info(app: AppHandle, url: String) -> Result<String, String> {
    let url = Path::new(&url);

    if !url.exists() {
        return Err("File does not exist".to_string());
    }

    println!("Open {}", url.display());
    let url_str = url.to_string_lossy().to_string();

    thread::spawn({
        let app_handle = app.clone();
        move || match libmediatool::open(&url_str) {
            Ok(meida_info) => {
                if let Err(e) = app_handle.emit("MediaInfo", meida_info) {
                    eprintln!("Error emitting MediaInfo event: {}", e);
                }
            }
            Err(e) => {
                let error_message = format!("Failed to open media info: {}", e);
                if let Err(e) = app_handle.emit("MediaInfo", error_message) {
                    eprintln!("Error emitting MediaInfo event: {}", e);
                }
            }
        }
    });
    return Ok("Success".to_string());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_media_info])
        .setup(|app| {
            app.set_menu(MenuBuilder::new(app).items(&[]).build()?)?;

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;

                // let window = app.get_webview_window("main").unwrap();
                // window.open_devtools();
                // window.close_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
