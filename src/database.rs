use std::env;

use anyhow::Context;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct Database {
    pg_pool: PgPool,
}

impl Database {
    pub async fn build() -> anyhow::Result<Database> {
        let pg_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE_URL").context("DATBASE_URL was not found")?)
            .await
            .context("error while trying connect to database")?;
        Ok(Database { pg_pool: pg_pool })
    }
}
