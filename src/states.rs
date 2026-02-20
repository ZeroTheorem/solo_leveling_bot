#[derive(Clone, Default)]
pub enum UserState {
    #[default]
    NoState,
    ReceiveTrainingName {
        training_id: i32,
    },
    DoReps {
        training_id: i32,
        exercise_name: String,
    },
    CompletingTraining {
        training_id: i32,
        exercise_name: String,
    },
    DeletingTraining,
}
