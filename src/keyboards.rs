use teloxide::types::{KeyboardButton, KeyboardMarkup};

pub fn create_keyboard() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![vec![KeyboardButton::new("Начать тренировку 🚀")]])
        .resize_keyboard(true)
}
