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
        .reply_markup(keyboards::create_keyboard())
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
                    bot.send_message(msg.chat.id, "Введи назнавание первого упражнения")
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
) -> anyhow::Result<()> {
    if let Some(text) = msg.text() {
        dialogue
            .update(UserState::DoReps {
                training_id: training_id,
                training_name: text.to_string(),
            })
            .await?;
        bot.send_message(msg.chat.id, format!("погнали {text}"))
            .await?;
    }
    Ok(())
}

pub async fn do_reps(
    bot: DefaultParseMode<Bot>,
    (training_id, training_name): (i32, String),
    msg: Message,
) -> anyhow::Result<()> {
    if let Some(_) = msg.text() {
        bot.send_message(msg.chat.id, format!("{training_id}--{training_name}"))
            .await?;
    }
    Ok(())
}
