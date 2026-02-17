use chrono::Local;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, KeyboardRemove,
};

use crate::database::Trainings;

pub fn create_main_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![vec![
        KeyboardButton::new("Начать тренировку 🚀"),
        KeyboardButton::new("Последние тренировки 📔"),
    ]])
    .resize_keyboard(true)
}

pub fn create_training_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![KeyboardButton::new("Сменить упражнение 🔄")],
        vec![KeyboardButton::new("Завершить тренировку 🏁")],
    ])
    .resize_keyboard(true)
}

pub fn dynamic(data: Vec<Trainings>) -> anyhow::Result<InlineKeyboardMarkup> {
    let mut buttons = vec![];

    for button in data {
        buttons.push(vec![InlineKeyboardButton::callback(
            button
                .created_at
                .unwrap_or(Local::now().naive_local())
                .format("%d.%m.%Y")
                .to_string(),
            button.id.to_string(),
        )])
    }
    Ok(InlineKeyboardMarkup::new(buttons))
}

pub fn off_menu() -> KeyboardRemove {
    KeyboardRemove::new()
}
