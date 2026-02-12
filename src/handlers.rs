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
    bot.send_message(msg.chat.id, message_provider.greetings_message()?)
        .await?;
    Ok(())
}
