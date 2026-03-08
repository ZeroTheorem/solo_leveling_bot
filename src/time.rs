use std::time::Instant;

pub fn since(start_time: Instant) -> (u64, u64, u64) {
    let elapsed_time = start_time.elapsed().as_secs();

    let hours = elapsed_time / 3600;
    let minutes = (elapsed_time % 3600) / 60;
    let seconds = elapsed_time % 60;

    (hours, minutes, seconds)
}
