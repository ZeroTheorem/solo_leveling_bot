use std::sync::Arc;

use crate::{database::Database, messages::MessageProvider, states::UserState};

use teloxide::{
    Bot,
    adaptors::DefaultParseMode,
    dispatching::dialogue::InMemStorage,
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
        .await?;
    }
    Ok(())
}
