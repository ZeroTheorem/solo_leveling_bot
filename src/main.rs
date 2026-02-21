use std::sync::Arc;

use commands::Commands;
use dotenv::dotenv;
use teloxide::dispatching::HandlerExt;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::types::ParseMode;
use teloxide::{filter_command, prelude::*};
mod commands;
mod database;
mod experience;
mod handlers;
mod keyboards;
mod messages;
mod states;
use states::UserState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load env variables
    dotenv().ok();

    //init database connection
    let db = database::Database::build().await?;

    // init message provider
    let message_provider = messages::MessageProvider::build()?;

    // log init
    let file_appender = tracing_appender::rolling::weekly("./logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::ERROR)
        .init();

    // create handlers
    let command_handler = filter_command::<Commands, _>()
        .branch(dptree::case![Commands::Start])
        .endpoint(handlers::start);

    let call_back_query_handler = Update::filter_callback_query().endpoint(handlers::call_back);

    let message_handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<states::UserState>, states::UserState>()
        .branch(command_handler)
        .branch(dptree::case![UserState::NoState].endpoint(handlers::any_text))
        .branch(
            dptree::case![UserState::ReceiveTrainingName { training_id }]
                .endpoint(handlers::receive_training_name),
        )
        .branch(
            dptree::case![UserState::DoReps {
                training_id,
                exercise_name
            }]
            .endpoint(handlers::do_reps),
        )
        .branch(
            dptree::case![UserState::CompletingTraining {
                training_id,
                exercise_name
            }]
            .endpoint(handlers::completing_training),
        )
        .branch(dptree::case![UserState::DeletingTraining].endpoint(handlers::deleting_training));

    // create Bot
    let bot = Bot::from_env().parse_mode(ParseMode::Html);

    Dispatcher::builder(
        bot,
        dptree::entry()
            .branch(message_handler)
            .branch(call_back_query_handler),
    )
    .dependencies(dptree::deps![
        Arc::new(db),
        Arc::new(message_provider),
        InMemStorage::<states::UserState>::new()
    ])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
    Ok(())
}
