use serde::{Serialize};
use tauri::{Manager, State};
use std::{thread, time::Duration};

pub struct SomeState {}

#[derive(Serialize)]
pub struct SomeError {
    message: String,
    reauth: bool,
}

#[tauri::command]
async fn do_http(_state: State<'_, SomeState>) -> Result<String, SomeError> {
    Ok("".to_owned())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            do_http])
        .manage(SomeState {})
        .build(tauri::generate_context!())
        .expect("error");

    app.run_iteration(|_app, _event| {});
    let _main_window = app.get_webview_window("main").unwrap();
    loop {
        app.run_iteration(|_app, _event| {});
        thread::sleep(Duration::from_millis(1));
        if app.webview_windows().is_empty() {
            app.cleanup_before_exit();
            break;
        }
    }
}
