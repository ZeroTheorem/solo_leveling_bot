use chrono::Local;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup};

use crate::database::Training;

pub const START_TRAINING_BTN: &str = "Start training 🚀";
pub const LAST_TRAININGS_BTN: &str = "Last trainings 📔";
pub const MY_PROGRESS_BTN: &str = "My progress 📈";
pub const DELETE_LAST_TRAINING_BTN: &str = "Delete last training ❌";
pub const SWITCH_EXERCISE_BTN: &str = "Switch exercise 🔄";
pub const DELETE_LAST_EXERCISE_BTN: &str = "Delete last exercise ❌";
pub const COMPLETE_TRAINING_BTN: &str = "Complete training 🏁";
pub const SHOW_HIGHEST_SET_BTN: &str = "My best set ⚡️";
pub const YES_BTN: &str = "Yes ✅";
pub const NO_BTN: &str = "No 🚫";

pub fn create_main_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new(START_TRAINING_BTN),
            KeyboardButton::new(LAST_TRAININGS_BTN),
        ],
        vec![
            KeyboardButton::new(MY_PROGRESS_BTN),
            KeyboardButton::new(SHOW_HIGHEST_SET_BTN),
        ],
        vec![KeyboardButton::new(DELETE_LAST_TRAINING_BTN)],
    ])
    .resize_keyboard(true)
}

pub fn create_training_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![
            KeyboardButton::new(SWITCH_EXERCISE_BTN),
            KeyboardButton::new(DELETE_LAST_EXERCISE_BTN),
        ],
        vec![KeyboardButton::new(COMPLETE_TRAINING_BTN)],
    ])
    .resize_keyboard(true)
}

pub fn specifying_question_menu() -> KeyboardMarkup {
    KeyboardMarkup::new(vec![
        vec![KeyboardButton::new(YES_BTN)],
        vec![KeyboardButton::new(NO_BTN)],
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
pub fn dynamic(data: Vec<Training>) -> anyhow::Result<InlineKeyboardMarkup> {
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
