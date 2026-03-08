use crate::database::Exercise;
use ::tera::Tera;
use anyhow::Context;
use std::fmt::Write;

pub struct MessageProvider {
    tera: Tera,
}

impl MessageProvider {
    pub fn build() -> anyhow::Result<MessageProvider> {
        let mut tera = Tera::default();
        tera.add_raw_template("greetings", include_str!("./texts/greetings.tera"))
            .context("error while adding template 'greetings'")?;

        tera.add_raw_template(
            "start_training",
            include_str!("./texts/start_training.tera"),
        )
        .context("error while adding template 'start_training'")?;

        tera.add_raw_template(
            "delete_last_exercise",
            include_str!("./texts/delete_last_exercise.tera"),
        )
        .context("error while adding template 'delete_last_rep'")?;

        tera.add_raw_template(
            "reps_completed",
            include_str!("./texts/reps_completed.tera"),
        )
        .context("error while adding template 'reps_completed'")?;

        tera.add_raw_template(
            "exercise_selected",
            include_str!("./texts/exercise_selected.tera"),
        )
        .context("error while adding template 'elapsed_time'")?;

        tera.add_raw_template("elapsed_time", include_str!("./texts/elapsed_time.tera"))
            .context("error while adding template 'exercise_selected'")?;

        tera.add_raw_template(
            "change_exercise",
            include_str!("./texts/change_exercise.tera"),
        )
        .context("error while adding template 'change_exercise'")?;

        tera.add_raw_template("get_journal", include_str!("./texts/get_journal.tera"))
            .context("error while adding template 'get_journal'")?;

        tera.add_raw_template("user_progress", include_str!("./texts/user_progress.tera"))
            .context("error while adding template 'user_progress'")?;

        Ok(MessageProvider { tera: tera })
    }
    pub fn greetings_message(&self, user_name: &str) -> anyhow::Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("user_name", user_name);
        let message = self
            .tera
            .render("greetings", &ctx)
            .context("error while render template 'greetings'")?;
        Ok(message)
    }

    pub fn start_training_message(&self, user_name: &str) -> anyhow::Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("user_name", user_name);
        let message = self
            .tera
            .render("start_training", &ctx)
            .context("error while render template 'start_training'")?;
        Ok(message)
    }

    pub fn elapsed_time_message(
        &self,
        (hours, minutes, seconds): (u64, u64, u64),
    ) -> anyhow::Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("hours", &hours);
        ctx.insert("minutes", &minutes);
        ctx.insert("seconds", &seconds);

        let message = self
            .tera
            .render("elapsed_time", &ctx)
            .context("error while render template 'elapsed_time'")?;

        Ok(message)
    }
    pub fn exercise_selected_message(&self, exercise_name: &str) -> anyhow::Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("exercise_name", exercise_name);
        let message = self
            .tera
            .render("exercise_selected", &ctx)
            .context("error while render template 'exercise_selected'")?;
        Ok(message)
    }

    pub fn change_exercise_message(&self, exercise_name: &str) -> anyhow::Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("exercise_name", exercise_name);
        let message = self
            .tera
            .render("change_exercise", &ctx)
            .context("error while render template 'change_exercise'")?;
        Ok(message)
    }

    pub fn wrong_format_message(&self) -> &str {
        include_str!("texts/wrong_format.txt")
    }

    pub fn success_delete_message(&self) -> &str {
        include_str!("texts/success_delete.txt")
    }
    pub fn cancel_delete_message(&self) -> &str {
        include_str!("texts/cancel_delete.txt")
    }
    pub fn not_found_training(&self) -> &str {
        include_str!("texts/not_found_training.txt")
    }
    pub fn cancel_completing_message(&self) -> &str {
        include_str!("texts/cancel_completing.txt")
    }
    pub fn delete_empty_training_message(&self) -> &str {
        include_str!("texts/delete_empty_training.txt")
    }
    pub fn complete_empty_training(&self) -> &str {
        include_str!("texts/complete_empty_training.txt")
    }
    pub fn empty_training_message(&self) -> &str {
        include_str!("texts/empty_training.txt")
    }
    pub fn no_history_found_message(&self) -> &str {
        include_str!("texts/no_found_history.txt")
    }
    pub fn no_set_to_delete_message(&self) -> &str {
        include_str!("texts/no_set_to_delete.txt")
    }
    pub fn get_journal_message(&self, user_name: &str) -> anyhow::Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("user_name", user_name);
        let message = self
            .tera
            .render("get_journal", &ctx)
            .context("error while render template 'get_journal'")?;
        Ok(message)
    }

    pub fn reps_completed_message(
        &self,
        weight: i32,
        reps: i32,
        exp_gained: i32,
    ) -> anyhow::Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("weight", &weight);
        ctx.insert("reps", &reps);
        ctx.insert("exp_gained", &exp_gained);
        let message = self
            .tera
            .render("reps_completed", &ctx)
            .context("error while render template 'get_journal'")?;
        Ok(message)
    }
    pub fn delete_last_rep_message(&self, exp_lost: i32) -> anyhow::Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("exp_lost", &exp_lost);
        let message = self
            .tera
            .render("delete_last_exercise", &ctx)
            .context("error while render template 'delete_last_exercise'")?;
        Ok(message)
    }

    pub fn get_user_progress(
        &self,
        user_name: &str,
        current_lvl: i64,
        current_user_exp: i64,
        exp_to_the_next_lvl: i64,
        progress_bar: String,
        percent: f64,
    ) -> anyhow::Result<String> {
        let mut ctx = tera::Context::new();
        ctx.insert("user_name", user_name);
        ctx.insert("current_lvl", &current_lvl);
        ctx.insert("current_user_exp", &current_user_exp);
        ctx.insert("exp_to_the_next_lvl", &exp_to_the_next_lvl);
        ctx.insert("progress_bar", &progress_bar);
        ctx.insert("percent", &percent);
        let message = self
            .tera
            .render("user_progress", &ctx)
            .context("error while render template 'user_progress'")?;
        Ok(message)
    }

    pub fn get_confirm_deleting_message(&self) -> &str {
        include_str!("texts/confirm_action.txt")
    }
    pub fn exercise_name_to_long_message(&self) -> &str {
        include_str!("texts/exercise_name_to_long.txt")
    }
    pub fn weight_or_reps_to_high(&self) -> &str {
        include_str!("texts/weight_or_reps_to_high.txt")
    }
    pub fn get_confirm_completing_message(&self) -> &str {
        include_str!("texts/completing_training.txt")
    }
    pub fn full_training_message(&self, exercises: Vec<Exercise>) -> anyhow::Result<String> {
        let mut answer = String::new();
        let mut temp_name = &exercises[0].name;
        let mut total: i64 = 0;

        write!(answer, "\n🏹 <b>{}</b>\n\n", temp_name)?;

        for exercise in &exercises {
            let total_per_rep = exercise.weight * exercise.reps;

            if &exercise.name != temp_name {
                temp_name = &exercise.name;
                write!(answer, "\n🏹 <b>{}</b>\n\n", temp_name)?;
            }
            write!(
                answer,
                "⚔️ <code>{} kg × {} reps</code> — <b>{} EXP</b>\n",
                exercise.weight, exercise.reps, total_per_rep
            )?;
            total += total_per_rep as i64
        }
        write!(
            answer,
            "\n✨ <b>Total EXP Earned: {} EXP</b> — Keep leveling up! 🚀",
            total
        )?;
        Ok(answer)
    }
}
