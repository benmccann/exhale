use tauri::{Manager, WebviewWindow};

#[tauri::command]
fn set_window_opacity(window: WebviewWindow, opacity: f64) -> Result<(), String> {
    window.set_decorations(true).map_err(|e| e.to_string())?;
    #[cfg(target_os = "macos")]
    return window.set_opacity(opacity).map_err(|e| e.to_string());
    #[cfg(not(target_os = "macos"))]
    {
        // On Linux/Windows, we can try using the window effects API
        let _ = opacity; // suppress unused warning
        Ok(())
    }
}

#[tauri::command]
fn set_ignore_cursor_events(window: WebviewWindow, ignore: bool) -> Result<(), String> {
    window.set_ignore_cursor_events(ignore).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // Get the main window and configure it
      let window = app.get_webview_window("main").unwrap();

      // Set initial window to ignore mouse events (click-through)
      window.set_ignore_cursor_events(true).unwrap();

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![set_window_opacity, set_ignore_cursor_events])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
