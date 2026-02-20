use std::sync::Arc;

use crate::{
    database::Database, experience, keyboards, messages::MessageProvider, states::UserState,
};

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
                "Start training 🚀" => {
                    let training_id = database.create_training(user.id.0 as i32).await?;
                    dialogue
                        .update(UserState::ReceiveTrainingName { training_id })
                        .await?;
                    bot.send_message(
                        msg.chat.id,
                        message_provider.start_training_message(&user.first_name)?,
                    )
                    .reply_markup(keyboards::choose_exercise_menu())
                    .await?;
                }
                "My progress 📈" => {
                    let (user_lvl, user_exp) =
                        database.get_current_progress(user.id.0 as i32).await?;

                    let exp_to_the_next_lvl = experience::calculate_exp_to_the_next_lvl(user_lvl);

                    let percent =
                        experience::get_percent(user_exp as f64, exp_to_the_next_lvl as f64);

                    bot.send_message(
                        msg.chat.id,
                        message_provider.get_user_progress(
                            &user.first_name,
                            user_lvl,
                            user_exp,
                            exp_to_the_next_lvl,
                            experience::generate_progress_bar(percent),
                            percent,
                        )?,
                    )
                    .await?;
                }
                "Last trainings 📔" => {
                    let last_five_training =
                        database.get_last_five_training(user.id.0 as i32).await?;
                    bot.send_message(
                        msg.chat.id,
                        message_provider.get_journal_message(&user.first_name)?,
                    )
                    .reply_markup(keyboards::dynamic(last_five_training)?)
                    .await?;
                }
                "Delete last training ❌" => {
                    dialogue.update(UserState::DeletingTraining).await?;
                    bot.send_message(msg.chat.id, message_provider.get_confirm_deleting_message())
                        .reply_markup(keyboards::specifying_question_menu())
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
            "Switch exercise 🔄" => {
                dialogue
                    .update(UserState::ReceiveTrainingName {
                        training_id: training_id,
                    })
                    .await?;

                bot.send_message(
                    msg.chat.id,
                    message_provider.change_exercise_message(&exercise_name)?,
                )
                .reply_markup(keyboards::choose_exercise_menu())
                .await?;
            }
            "Compleat training 🏁" => {
                dialogue
                    .update(UserState::CompletingTraining {
                        training_id,
                        exercise_name,
                    })
                    .await?;
                bot.send_message(
                    msg.chat.id,
                    message_provider.get_confirm_completing_message(),
                )
                .reply_markup(keyboards::specifying_question_menu())
                .await?;
            }
            "Delete last exercise ❌" => {
                let (weight, reps) = database.delete_last_exercise(training_id).await?;
                bot.send_message(
                    msg.chat.id,
                    message_provider.delete_last_rep_message(weight * reps)?,
                )
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
                        bot.send_message(
                            msg.chat.id,
                            message_provider.reps_completed_message(
                                parsed_input[0],
                                parsed_input[1],
                                parsed_input[0] * parsed_input[1],
                            )?,
                        )
                        .await?;
                    }
                    _ => {
                        bot.send_message(msg.chat.id, message_provider.wrong_format_message())
                            .await?;
                        return Ok(());
                    }
                };
            }
        }
    }
    Ok(())
}

pub async fn completing_training(
    bot: DefaultParseMode<Bot>,
    dialogue: MyDialogue,
    (training_id, exercise_name): (i32, String),
    message_provider: Arc<MessageProvider>,
    database: Arc<Database>,
    msg: Message,
) -> anyhow::Result<()> {
    if let Some(text) = msg.text() {
        if let Some(user) = msg.from() {
            match text {
                "Yes ✅" => {
                    dialogue.update(UserState::NoState).await?;

                    let (current_lvl, current_exp) =
                        database.get_current_progress(user.id.0 as i32).await?;

                    let gained_exp = database.get_total_exp_fro_training(training_id).await?;

                    let (new_user_lvl, new_user_exp) = experience::update_user_progress(
                        current_lvl,
                        current_exp,
                        gained_exp as i32,
                    );

                    database
                        .update_user_progress(new_user_lvl, new_user_exp, user.id.0 as i32)
                        .await?;

                    let exercises = database.get_exercises_from_training(training_id).await?;

                    let exp_to_the_next_lvl =
                        experience::calculate_exp_to_the_next_lvl(new_user_lvl);

                    let percent =
                        experience::get_percent(new_user_exp as f64, exp_to_the_next_lvl as f64);

                    bot.send_message(
                        msg.chat.id,
                        message_provider.full_training_message(exercises)?,
                    )
                    .await?;

                    bot.send_message(
                        msg.chat.id,
                        message_provider.get_user_progress(
                            &user.first_name,
                            new_user_lvl,
                            new_user_exp,
                            exp_to_the_next_lvl,
                            experience::generate_progress_bar(percent),
                            percent,
                        )?,
                    )
                    .reply_markup(keyboards::create_main_menu())
                    .await?;
                }
                "No 🚫" => {
                    dialogue
                        .update(UserState::DoReps {
                            training_id,
                            exercise_name,
                        })
                        .await?;
                    bot.send_message(msg.chat.id, "Ok let continue")
                        .reply_markup(keyboards::create_training_menu())
                        .await?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

pub async fn deleting_training(
    bot: DefaultParseMode<Bot>,
    dialogue: MyDialogue,
    message_provider: Arc<MessageProvider>,
    database: Arc<Database>,
    msg: Message,
) -> anyhow::Result<()> {
    if let Some(text) = msg.text() {
        if let Some(user) = msg.from() {
            match text {
                "Yes ✅" => {
                    let last_user_training_id =
                        database.get_last_user_training(user.id.0 as i32).await?;

                    let total_exp_earned_for_the_last_training = database
                        .get_total_exp_fro_training(last_user_training_id)
                        .await?;

                    database.delete_last_training(user.id.0 as i32).await?;

                    let (current_lvl, current_exp) =
                        database.get_current_progress(user.id.0 as i32).await?;

                    let (new_user_lvl, new_user_exp) = experience::downgrade_user_progress(
                        current_lvl,
                        current_exp,
                        total_exp_earned_for_the_last_training as i32,
                    );

                    database
                        .update_user_progress(new_user_lvl, new_user_exp, user.id.0 as i32)
                        .await?;

                    let exp_to_the_next_lvl =
                        experience::calculate_exp_to_the_next_lvl(new_user_lvl);

                    let percent =
                        experience::get_percent(new_user_exp as f64, exp_to_the_next_lvl as f64);

                    bot.send_message(
                        msg.chat.id,
                        message_provider.get_user_progress(
                            &user.first_name,
                            new_user_lvl,
                            new_user_exp,
                            exp_to_the_next_lvl,
                            experience::generate_progress_bar(percent),
                            percent,
                        )?,
                    )
                    .reply_markup(keyboards::create_main_menu())
                    .await?;
                }
                "No 🚫" => {
                    dialogue.update(UserState::NoState).await?;
                    bot.send_message(msg.chat.id, "Ok not deleting")
                        .reply_markup(keyboards::create_main_menu())
                        .await?;
                }
                _ => {}
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
    bot.answer_callback_query(q.id).await?;
    let training_id: i32 = q
        .data
        .ok_or(anyhow::anyhow!("call_back не отсутствует"))?
        .parse()?;
    let exercises = database.get_exercises_from_training(training_id).await?;
    if let Some(message) = q.message {
        bot.send_message(
            message.chat.id,
            message_provider.full_training_message(exercises)?,
        )
        .await?;
    }

    Ok(())
}
