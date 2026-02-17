use std::sync::Arc;

use crate::{database::Database, keyboards, messages::MessageProvider, states::UserState};

use teloxide::{
    Bot,
    adaptors::DefaultParseMode,
    dispatching::dialogue::InMemStorage,
    payloads::SendMessageSetters,
    prelude::{Dialogue, Requester},
    types::{CallbackQuery, Message},
};

type MyDialogue = Dialogue<UserState, InMemStorage<UserState>>;

pub async fn start(
    bot: DefaultParseMode<Bot>,
    msg: Message,
    database: Arc<Database>,
    message_provider: Arc<MessageProvider>,
) -> anyhow::Result<()> {
    if let Some(user) = msg.from() {
        database.create_user(user.id.0 as i32).await?;
        bot.send_message(
            msg.chat.id,
            message_provider.greetings_message(&user.first_name)?,
        )
        .reply_markup(keyboards::create_main_menu())
        .await?;
    }
    Ok(())
}

pub async fn any_text(
    bot: DefaultParseMode<Bot>,
    msg: Message,
    database: Arc<Database>,
    dialogue: MyDialogue,
    message_provider: Arc<MessageProvider>,
) -> anyhow::Result<()> {
    if let Some(text) = msg.text() {
        if let Some(user) = msg.from() {
            match text {
                "Начать тренировку 🚀" => {
                    let training_id = database.create_training(user.id.0 as i32).await?;
                    dialogue
                        .update(UserState::ReceiveTrainingName { training_id })
                        .await?;
                    bot.send_message(
                        msg.chat.id,
                        message_provider.start_training_message(&user.first_name)?,
                    )
                    .reply_markup(keyboards::off_menu())
                    .await?;
                }
                "Последние тренировки 📔" => {
                    let last_five_training =
                        database.get_last_five_training(user.id.0 as i32).await?;
                    bot.send_message(
                        msg.chat.id,
                        message_provider.get_journal_message(&user.first_name)?,
                    )
                    .reply_markup(keyboards::dynamic(last_five_training)?)
                    .await?;
                }
                _ => (),
            }
        }
    }
    Ok(())
}

pub async fn receive_training_name(
    bot: DefaultParseMode<Bot>,
    msg: Message,
    training_id: i32,
    dialogue: MyDialogue,
    message_provider: Arc<MessageProvider>,
) -> anyhow::Result<()> {
    if let Some(text) = msg.text() {
        dialogue
            .update(UserState::DoReps {
                training_id: training_id,
                exercise_name: text.to_string(),
            })
            .await?;
        bot.send_message(
            msg.chat.id,
            message_provider.exercise_selected_message(text)?,
        )
        .reply_markup(keyboards::create_training_menu())
        .await?;
    }
    Ok(())
}

pub async fn do_reps(
    bot: DefaultParseMode<Bot>,
    dialogue: MyDialogue,
    (training_id, exercise_name): (i32, String),
    message_provider: Arc<MessageProvider>,
    database: Arc<Database>,
    msg: Message,
) -> anyhow::Result<()> {
    if let Some(text) = msg.text() {
        match text {
            "Сменить упражнение 🔄" => {
                dialogue
                    .update(UserState::ReceiveTrainingName {
                        training_id: training_id,
                    })
                    .await?;

                bot.send_message(
                    msg.chat.id,
                    message_provider.change_exercise_message(&exercise_name)?,
                )
                .reply_markup(keyboards::off_menu())
                .await?;
            }
            "Завершить тренировку 🏁" => {
                dialogue.update(UserState::NoState).await?;
                bot.send_message(msg.chat.id, "Тренировка завершена!")
                    .reply_markup(keyboards::create_main_menu())
                    .await?;
            }
            _ => {
                let user_input: Result<Vec<i32>, _> =
                    text.split_whitespace().map(str::parse).collect();
                match user_input {
                    Ok(parsed_input) if parsed_input.len() == 2 => {
                        database
                            .create_exercise(
                                &exercise_name,
                                parsed_input[0],
                                parsed_input[1],
                                training_id,
                            )
                            .await?;
                        bot.send_message(msg.chat.id, "OK").await?;
                    }
                    _ => {
                        bot.send_message(msg.chat.id, message_provider.wrong_format_message()?)
                            .await?;
                        return Ok(());
                    }
                };
            }
        }
    }
    Ok(())
}

pub async fn call_back(
    bot: DefaultParseMode<Bot>,
    q: CallbackQuery,
    message_provider: Arc<MessageProvider>,
    database: Arc<Database>,
) -> anyhow::Result<()> {
    let training_id: i32 = q
        .data
        .ok_or(anyhow::anyhow!("call_back не отсутствует"))?
        .parse()?;
    let exercises = database.get_exercises_from_training(training_id).await?;
    if let Some(message) = q.message {
        bot.edit_message_text(
            message.chat.id,
            message.id,
            message_provider.full_training_message(exercises)?,
        )
        .await?;
    }

    Ok(())
}
