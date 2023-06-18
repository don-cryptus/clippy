use once_cell::sync::OnceCell;
use tauri::{LogicalSize, Manager};

use super::clipboard::clipboard_helper::ClipboardHelper;

pub static MAIN_WINDOW_X: i32 = 375;
pub static MAIN_WINDOW_Y: i32 = 600;

pub static APP: OnceCell<tauri::AppHandle> = OnceCell::new();
pub static CLIPBOARD_HELPER: OnceCell<ClipboardHelper> = OnceCell::new();

pub fn setup(app: &mut tauri::App) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let window = app.get_window("main").unwrap();
    let clipboard_helper: ClipboardHelper = ClipboardHelper::new();

    let _ = window.set_size(LogicalSize::new(MAIN_WINDOW_X, MAIN_WINDOW_Y));

    #[cfg(debug_assertions)]
    {
        window.open_devtools();
    }

    APP.set(app.handle()).expect("error initializing tauri app");
    CLIPBOARD_HELPER
        .set(clipboard_helper)
        .expect("error initializing clipboard helper");

    Ok(())
}
