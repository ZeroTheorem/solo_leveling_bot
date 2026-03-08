use std::time::Instant;

#[derive(Clone, Default)]
pub enum UserState {
    #[default]
    NoState,
    ReceiveTrainingName {
        training_id: i64,
        start_training_time: Instant,
    },
    DoReps {
        training_id: i64,
        exercise_name: String,
        start_training_time: Instant,
    },
    CompletingTraining {
        training_id: i64,
        exercise_name: String,
        start_training_time: Instant,
    },
    DeletingTraining,
}
