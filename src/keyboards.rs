use chrono::Local;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, KeyboardRemove,
};

use crate::database::Trainings;

pub fn create_main_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("Start training 🚀"),
            KeyboardButton::new("Last trainings 📔"),
        ],
        vec![KeyboardButton::new("My progress 📈")],
    ])
    .resize_keyboard(true)
}

pub fn create_training_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("Switch exercise 🔄"),
            KeyboardButton::new("Delete last exercise ❌"),
        ],
        vec![KeyboardButton::new("Compleat training 🏁")],
    ])
    .resize_keyboard(true)
}

pub fn choose_exercise_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new("Bench press"),
            KeyboardButton::new("Lat pull-down"),
        ],
        vec![
            KeyboardButton::new("Incline bench press"),
            KeyboardButton::new("Lat pull-down hummer"),
        ],
        vec![
            KeyboardButton::new("EZ biceps curls"),
            KeyboardButton::new("Triceps push down"),
        ],
        vec![
            KeyboardButton::new("Biceps curls"),
            KeyboardButton::new("Single triceps push down"),
        ],
        vec![
            KeyboardButton::new("Biceps curls hummer"),
            KeyboardButton::new("Horizontal row hummer"),
        ],
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
