mod day01;
mod day02;
mod day03;

enum Level {
    Easy,
    Hard,
}

fn main() {
    let day = 3;
    let level = Level::Hard;

    match day {
        1 => match level {
            Level::Easy => day01::historian_hysteria_easy(),
            Level::Hard => day01::historian_hysteria_hard(),
        },
        2 => match level {
            Level::Easy => day02::rednosed_reports_easy(),
            Level::Hard => day02::rednosed_reports_hard(),
        },
        3 => match level {
            Level::Easy => day03::mull_it_over_easy(),
            Level::Hard => day03::mull_it_over_hard(),
        },
        _ => panic!("Unknown day!"),
    }
}
