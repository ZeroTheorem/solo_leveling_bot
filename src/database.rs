use std::env;

use anyhow::Context;
use sqlx::{PgPool, postgres::PgPoolOptions, types::chrono::NaiveDateTime};

pub struct Database {
    pg_pool: PgPool,
}

pub struct Trainings {
    pub id: i64,
    pub created_at: Option<NaiveDateTime>,
}

pub struct Exercises {
    pub name: String,
    pub weight: i64,
    pub reps: i64,
}

impl Database {
    pub async fn build() -> anyhow::Result<Database> {
        let pg_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE_URL").context("DATABASE_URL was not found")?)
            .await
            .context("error while trying connect to database")?;
        Ok(Database { pg_pool: pg_pool })
    }

    pub async fn create_user(&self, telegram_id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO users (telegram_id, lvl, exp) VALUES ($1, 0, 0)
             ON CONFLICT (telegram_id) DO NOTHING;",
            telegram_id
        )
        .execute(&self.pg_pool)
        .await
        .context("error while create user")?;
        Ok(())
    }

    pub async fn create_training(&self, owner_id: i32) -> anyhow::Result<i32> {
        let training_id = sqlx::query!(
            "INSERT INTO training (owner_id) VALUES ($1) RETURNING id",
            owner_id
        )
        .fetch_one(&self.pg_pool)
        .await
        .context("error while create training")?;
        Ok(training_id.id)
    }

    pub async fn create_exercise(
        &self,
        name: &str,
        weight: i32,
        reps: i32,
        training_id: i32,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO exercises (name, weight, reps, training_id) VALUES ($1, $2, $3, $4);",
            name,
            weight,
            reps,
            training_id
        )
        .execute(&self.pg_pool)
        .await
        .context("error while create exercise")?;
        Ok(())
    }

    pub async fn get_last_five_training(&self, owner_id: i32) -> anyhow::Result<Vec<Trainings>> {
        let last_five_training = sqlx::query_as!(
            Trainings,
            "SELECT id, created_at FROM training WHERE owner_id = $1 ORDER BY created_at DESC LIMIT 5;",
            owner_id
        )
        .fetch_all(&self.pg_pool)
        .await
        .context("error while get trainings")?;
        Ok(last_five_training)
    }

    pub async fn get_exercises_from_training(
        &self,
        training_id: i32,
    ) -> anyhow::Result<Vec<Exercises>> {
        let exercises = sqlx::query_as!(
            Exercises,
            "SELECT name, weight, reps FROM exercises WHERE training_id = $1",
            training_id
        )
        .fetch_all(&self.pg_pool)
        .await
        .context("error while get exercises")?;
        Ok(exercises)
    }

    pub async fn get_total_exp_fro_training(&self, training_id: i32) -> anyhow::Result<i64> {
        let total_exp = sqlx::query!(
            "SELECT SUM(weight*reps) as total_exp FROM exercises WHERE training_id = $1;",
            training_id
        )
        .fetch_one(&self.pg_pool)
        .await
        .context("error while get exp for training")?;
        Ok(total_exp.total_exp.unwrap_or(0))
    }

    pub async fn get_current_progress(&self, user_id: i32) -> anyhow::Result<(i32, i32)> {
        let current_progress = sqlx::query!(
            "SELECT lvl, exp FROM users WHERE telegram_id = $1;",
            user_id
        )
        .fetch_one(&self.pg_pool)
        .await
        .context("error while get current progress")?;
        Ok((current_progress.lvl, current_progress.exp))
    }

    pub async fn update_user_progress(
        &self,
        lvl: i32,
        exp: i32,
        user_id: i32,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            "UPDATE users SET lvl = $1, exp = $2 WHERE telegram_id = $3",
            lvl,
            exp,
            user_id
        )
        .execute(&self.pg_pool)
        .await
        .context("error while update user progress")?;
        Ok(())
    }

    pub async fn delete_last_exercise(&self, training_id: i32) -> anyhow::Result<(i32, i32)> {
        let last_exercise = sqlx::query!(
            "DELETE FROM exercises WHERE id = (SELECT MAX(id) FROM exercises WHERE training_id = $1) RETURNING weight, reps",
            training_id
        )
        .fetch_one(&self.pg_pool)
        .await
        .context("error while delete last rep")?;
        Ok((last_exercise.weight, last_exercise.reps))
    }
}
