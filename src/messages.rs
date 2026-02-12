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
        Ok(MessageProvider { tera: tera })
    }
    pub fn greetings_message(&self) -> anyhow::Result<String> {
        let ctx = tera::Context::new();
        let message = self
            .tera
            .render("greetings", &ctx)
            .context("error while render template 'greetings'")?;
        Ok(message)
    }
}
