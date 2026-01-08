pub mod crypto;

#[tauri::command]
fn log_message(message: &str) {
    log::info!("{}", message);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    tauri_plugin_log::LogTarget::Stdout,
                    tauri_plugin_log::LogTarget::Webview,
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![log_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
