use std::sync::Arc;

use crate::{database::Database, keyboards, messages::MessageProvider, states::UserState};

use teloxide::{
    Bot,
    adaptors::DefaultParseMode,
    dispatching::dialogue::InMemStorage,
    payloads::SendMessageSetters,
    prelude::{Dialogue, Requester},
    types::Message,
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
        match text {
            "Начать тренировку 🚀" => {
                if let Some(user) = msg.from() {
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
            }
            _ => (),
        }
    }
    Ok(())
}

pub async fn receive_traning_name(
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
            _ => {
                let user_input: Result<Vec<i32>, _> =
                    text.split_whitespace().map(str::parse).collect();
                let parsed_user_input = match user_input {
                    Ok(v) if v.len() == 2 => v,
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
