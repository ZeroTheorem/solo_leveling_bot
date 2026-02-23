use std::env;

use anyhow::Context;
use sqlx::{PgPool, postgres::PgPoolOptions, types::chrono::NaiveDateTime};

pub struct Database {
    pg_pool: PgPool,
}

pub struct Training {
    pub id: i64,
    pub created_at: Option<NaiveDateTime>,
}

pub struct Exercise {
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

    pub async fn create_user(&self, telegram_id: i64) -> anyhow::Result<()> {
        sqlx::query!(
            "INSERT INTO users (telegram_id, lvl, exp) VALUES ($1, 1, 0)
             ON CONFLICT (telegram_id) DO NOTHING;",
            telegram_id
        )
        .execute(&self.pg_pool)
        .await
        .context("error while create user")?;
        Ok(())
    }

    pub async fn create_training(&self, owner_id: i64) -> anyhow::Result<i64> {
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
        training_id: i64,
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

    pub async fn delete_last_training(&self, user_id: i64) -> anyhow::Result<()> {
        sqlx::query!(
            "DELETE FROM training WHERE id = (SELECT MAX(id) FROM training WHERE owner_id = $1)",
            user_id
        )
        .execute(&self.pg_pool)
        .await
        .context("error delete training")?;
        Ok(())
    }
    pub async fn get_last_five_training(
        &self,
        owner_id: i64,
    ) -> anyhow::Result<Option<Vec<Training>>> {
        let last_five_training = sqlx::query_as!(
            Training,
            "SELECT id, created_at FROM training WHERE owner_id = $1 ORDER BY created_at DESC LIMIT 5;",
            owner_id
        )
        .fetch_all(&self.pg_pool)
        .await
        .context("error while get trainings")?;
        if last_five_training.len() == 0 {
            return Ok(None);
        }
        Ok(Some(last_five_training))
    }

    pub async fn get_exercises_from_training(
        &self,
        training_id: i64,
    ) -> anyhow::Result<Option<Vec<Exercise>>> {
        let exercises = sqlx::query_as!(
            Exercise,
            "SELECT name, weight, reps FROM exercises WHERE training_id = $1",
            training_id
        )
        .fetch_all(&self.pg_pool)
        .await
        .context("error while get exercises")?;
        if exercises.len() == 0 {
            return Ok(None);
        }
        Ok(Some(exercises))
    }

    pub async fn get_total_exp_for_training(
        &self,
        training_id: i64,
    ) -> anyhow::Result<Option<i64>> {
        let total_exp = sqlx::query!(
            "SELECT SUM(weight*reps) as total_exp FROM exercises WHERE training_id = $1;",
            training_id
        )
        .fetch_one(&self.pg_pool)
        .await
        .context("error while get exp for training")?;
        Ok(total_exp.total_exp)
    }

    pub async fn get_current_progress(&self, user_id: i64) -> anyhow::Result<(i64, i64)> {
        let current_progress = sqlx::query!(
            "SELECT lvl, exp FROM users WHERE telegram_id = $1;",
            user_id
        )
        .fetch_one(&self.pg_pool)
        .await
        .context("error while get current progress")?;
        Ok((current_progress.lvl, current_progress.exp))
    }
    pub async fn get_last_user_training(&self, user_id: i64) -> anyhow::Result<Option<i64>> {
        let last_training = sqlx::query!(
            "SELECT id FROM training WHERE id = (SELECT MAX(id) FROM training WHERE owner_id = $1)",
            user_id,
        )
        .fetch_optional(&self.pg_pool)
        .await
        .context("error while get last user training")?;
        match last_training {
            Some(last_training) => Ok(Some(last_training.id)),
            None => Ok(None),
        }
    }

    pub async fn update_user_progress(
        &self,
        lvl: i64,
        exp: i64,
        user_id: i64,
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

    pub async fn delete_last_exercise(
        &self,
        training_id: i64,
    ) -> anyhow::Result<Option<(i32, i32)>> {
        let last_exercise = sqlx::query!(
            "DELETE FROM exercises WHERE id = (SELECT MAX(id) FROM exercises WHERE training_id = $1) RETURNING weight, reps",
            training_id
        )
        .fetch_optional(&self.pg_pool)
        .await
        .context("error while delete last rep")?;
        match last_exercise {
            Some(last_exercise) => Ok(Some((last_exercise.reps, last_exercise.weight))),
            None => Ok(None),
        }
    }
}
