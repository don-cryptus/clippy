use crate::types::enums::HotkeyEvent;
use sea_orm::Iden;
use std::sync::LazyLock;

pub static GLOBAL_EVENTS: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        HotkeyEvent::WindowDisplayToggle.to_string(),
        HotkeyEvent::TypeClipboard.to_string(),
    ]
});

pub static MAIN_WINDOW_X: i32 = 375;
pub static MAIN_WINDOW_Y: i32 = 600;

pub static ABOUT_WINDOW_X: i32 = 375;
pub static ABOUT_WINDOW_Y: i32 = 600;

pub static SETTINGS_WINDOW_X: i32 = 500;
pub static SETTINGS_WINDOW_Y: i32 = 450;

pub static MAX_IMAGE_DIMENSIONS: u32 = 1280;
pub static MAX_TEXT_PREVIEW: usize = 500; // Adjust preview length as needed

pub static DISPLAY_SCALE: f32 = 1.0;
pub static DISPLAY_SCALE_MIN: f32 = 0.5;
pub static DISPLAY_SCALE_MAX: f32 = 2.0;

pub static MAX_FILE_SIZE: u32 = 10_485_760;
pub static MAX_FILE_SIZE_MIN: u32 = 0;
pub static MAX_FILE_SIZE_MAX: u32 = 104_857_600;

pub static MAX_IMAGE_SIZE: u32 = 10_485_760;
pub static MAX_IMAGE_SIZE_MIN: u32 = 0;
pub static MAX_IMAGE_SIZE_MAX: u32 = 104_857_600;

pub static MAX_TEXT_SIZE: u32 = 10_485_760;
pub static MAX_TEXT_SIZE_MIN: u32 = 0;
pub static MAX_TEXT_SIZE_MAX: u32 = 104_857_600;

pub static MAX_RTF_SIZE: u32 = 10_485_760;
pub static MAX_RTF_SIZE_MIN: u32 = 0;
pub static MAX_RTF_SIZE_MAX: u32 = 104_857_600;

pub static MAX_HTML_SIZE: u32 = 10_485_760;
pub static MAX_HTML_SIZE_MIN: u32 = 0;
pub static MAX_HTML_SIZE_MAX: u32 = 104_857_600;
