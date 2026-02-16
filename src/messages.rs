use ::tera::Tera;
use anyhow::Context;

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
}
