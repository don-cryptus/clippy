use crate::{
    connection,
    service::clipboard::upsert_db,
    utils::setup::{APP, CLIPBOARD_HELPER},
};
use arboard::Clipboard;
use entity::clipboard::{self, ActiveModel, Model};
use image::{ImageBuffer, RgbaImage};
use sea_orm::{EntityTrait, QueryOrder, Set};
use std::io::Cursor;
use tauri::{regex::Regex, Manager};

#[derive(Debug, Clone)]
pub struct ClipboardHelper {
    clipboard: (Option<String>, Option<RgbaImage>),
    model: Option<Model>,
    active_model: Option<ActiveModel>,
}
impl ClipboardHelper {
    pub fn new() -> Self {
        ClipboardHelper {
            clipboard: Self::get_os_clipboard(),
            model: None,
            active_model: None,
        }
    }

    fn get_os_clipboard() -> (Option<String>, Option<RgbaImage>) {
        let mut clipboard = Clipboard::new().unwrap();
        let text = clipboard.get_text().ok();
        let image = clipboard.get_image().ok();

        let image: Option<RgbaImage> = if image.is_some() {
            Some(
                ImageBuffer::from_raw(
                    image.clone().unwrap().width.try_into().unwrap(),
                    image.clone().unwrap().height.try_into().unwrap(),
                    image.clone().unwrap().bytes.into_owned(),
                )
                .unwrap(),
            )
        } else {
            None
        };

        (text, image)
    }

    pub fn refresh_clipboard(mut self) {
        let clipboard = Self::get_os_clipboard();
        self.clipboard = clipboard;
    }

    pub async fn upsert_clipboard() {
        let clipboard_helper = CLIPBOARD_HELPER.get().unwrap();

        if clipboard_helper.check_if_last_is_same().await {
            return;
        }

        let model = upsert_db().await;

        APP.get()
            .unwrap()
            .get_window("main")
            .unwrap()
            .emit("clipboard_listener", model.unwrap())
            .unwrap();
    }

    async fn check_if_last_is_same(&self) -> bool {
        let (text, image) = &self.clipboard;

        if text.is_none() && image.is_none() {
            println!("No clipboard data found?");
            return false;
        }

        let db = connection::establish_connection().await.unwrap();

        let last_clipboard = clipboard::Entity::find()
            .order_by_desc(clipboard::Column::Id)
            .one(&db)
            .await
            .unwrap();

        if last_clipboard.is_none() {
            println!("Last clipboard does not exist in db");
            return false;
        }

        let last_clipboard = last_clipboard.unwrap();

        if text.is_some() // check if text is same
        && last_clipboard.content.is_some()
        && text.clone().unwrap() == last_clipboard.content.as_deref().unwrap()
            || image.is_some() // check if image is same
            && last_clipboard.blob.is_some()
            && &image.clone().unwrap().to_vec() == last_clipboard.blob.as_deref().unwrap()
        {
            println!("Clipboard is the same as last clipboard");
            return true;
        }

        println!("clipboard and db are not the same");
        return false;
    }

    pub fn parse_model(self) -> ActiveModel {
        let (text, image) = self.clipboard;

        let re = Regex::new(r"^#?(?:[0-9a-f]{3}){1,2}$").unwrap();

        let r#type = if text.is_some() {
            Set("text".to_string())
        } else if text.is_some() && re.is_match(&text.as_deref().unwrap()) {
            Set("color".to_string())
        } else {
            Set("image".to_string())
        };

        let active_model = if image.is_none() {
            ActiveModel {
                ..Default::default()
            }
        } else {
            let mut bytes: Vec<u8> = Vec::new();
            image
                .as_ref()
                .unwrap()
                .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
                .unwrap();

            ActiveModel {
                size: Set(Some(bytes.len().to_string())),
                height: Set(Some(image.clone().unwrap().height() as i32)),
                width: Set(Some(image.unwrap().width() as i32)),
                blob: Set(Some(bytes)),
                ..Default::default()
            }
        };

        ActiveModel {
            r#type,
            content: Set(text),
            star: Set(Some(false)),
            ..active_model
        }
    }
}
