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

pub fn downgrade_user_progress(
    current_lvl: i32,
    current_exp: i32,
    downgrade_exp: i32,
) -> (i32, i32) {
    let mut new_user_lvl = current_lvl;
    let mut new_user_exp = current_exp;
    let mut temp_downgrade_exp = downgrade_exp;

    while temp_downgrade_exp > 0 {
        temp_downgrade_exp -= new_user_exp;
        if temp_downgrade_exp > 0 {
            if new_user_lvl == 1 {
                new_user_exp = 0;
                break;
            }
            new_user_lvl -= 1;
            new_user_exp = calculate_exp_to_the_next_lvl(new_user_lvl);
        }
        if temp_downgrade_exp < 0 {
            new_user_exp = temp_downgrade_exp * -1
        }
        if temp_downgrade_exp == 0 {
            new_user_exp = 0
        }
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
