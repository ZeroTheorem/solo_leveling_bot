use teloxide::types::{KeyboardButton, KeyboardMarkup, KeyboardRemove, ReplyMarkup};

pub fn create_main_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![vec![KeyboardButton::new("Начать тренировку 🚀")]])
        .resize_keyboard(true)
}

pub fn create_training_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![vec![
        KeyboardButton::new("Сменить упражнение 🔄"),
        KeyboardButton::new("Завершить тренировку 🏁"),
    ]])
    .resize_keyboard(true)
}

pub fn off_menu() -> KeyboardRemove {
    KeyboardRemove::new()
}
