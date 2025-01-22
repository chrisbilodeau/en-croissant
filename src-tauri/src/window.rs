use tauri::Manager;
use tauri_plugin_window_state::{AppHandleExt, StateFlags};

use crate::error::Error;

#[tauri::command]
#[specta::specta]
pub fn set_titlebar_decorations_and_restart(
    app: tauri::AppHandle,
    enable_decorations: bool,
) -> Result<(), Error> {
    let window = app
        .get_webview_window("main")
        .expect("app must have main window");
    window.set_decorations(enable_decorations)?;
    app.save_window_state(StateFlags::all())?;
    app.restart();
}
