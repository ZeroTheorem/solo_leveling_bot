use crate::database::Exercises;
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

        tera.add_raw_template("start_trainig", include_str!("./texts/start_training.tera"))
            .context("error while adding template 'start_trainig'")?;

        tera.add_raw_template(
            "exercise_selected",
            include_str!("./texts/exercise_selected.tera"),
        )
        .context("error while adding template 'exercise_selected'")?;

        tera.add_raw_template(
            "change_exercise",
            include_str!("./texts/change_exercise.tera"),
        )
        .context("error while adding template 'change_exercise'")?;

        tera.add_raw_template("wrong_format", include_str!("./texts/wrong_format.tera"))
            .context("error while adding template 'wrong_format'")?;

        tera.add_raw_template("get_journal", include_str!("./texts/get_journal.tera"))
            .context("error while adding template 'get_journal'")?;

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
            .render("start_trainig", &ctx)
            .context("error while render template 'start_trainig'")?;
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

    pub fn wrong_format_message(&self) -> anyhow::Result<String> {
        let ctx = tera::Context::new();
        let message = self
            .tera
            .render("wrong_format", &ctx)
            .context("error while render template 'wrong_format'")?;
        Ok(message)
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
    pub fn full_training_message(&self, exercises: Vec<Exercises>) -> anyhow::Result<String> {
        if exercises.is_empty() {
            return Ok("Нет данных по тренировке 💤".to_string());
        }

        let mut answer = String::new();
        let mut temp_name = &exercises[0].name;

        write!(answer, "<b>{}</b>\n\n", temp_name)?;

        for exercise in &exercises {
            if &exercise.name != temp_name {
                temp_name = &exercise.name;
                write!(answer, "\n<b>{}</b>\n\n", temp_name)?;
            }
            write!(
                answer,
                "{} kg -- {} reps. | 989xp\n",
                exercise.weight, exercise.reps
            )?;
        }
        Ok(answer)
    }
}
