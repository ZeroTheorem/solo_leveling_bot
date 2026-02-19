pub fn update_user_progress(current_lvl: i32, current_exp: i32, gained_exp: i32) -> (i32, i32) {
    let mut exp_for_next_lvl = calculate_exp_to_the_next_lvl(current_lvl);
    let mut new_user_lvl = current_lvl;
    let mut new_user_exp = current_exp + gained_exp;
    while new_user_exp >= exp_for_next_lvl {
        new_user_lvl += 1;
        new_user_exp = new_user_exp - exp_for_next_lvl;
        exp_for_next_lvl = calculate_exp_to_the_next_lvl(new_user_lvl);
    }
    (new_user_lvl, new_user_exp)
}

pub fn calculate_exp_to_the_next_lvl(current_lvl: i32) -> i32 {
    (current_lvl + 1) * (current_lvl + 1) * 10
}

pub fn generate_progress_bar(percent: f64) -> String {
    let completed = percent * 20.0 / 100.0;
    "█".repeat(completed as usize) + "░".repeat(20 - completed as usize).as_str()
}

pub fn get_percent(current_exp: f64, exp_to_the_next_lvl: f64) -> f64 {
    (current_exp / exp_to_the_next_lvl) * 100.0
}
