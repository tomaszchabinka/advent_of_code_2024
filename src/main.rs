mod day01;

enum Level {
    Easy,
    Hard,
}

fn main() {
    let day = 1;
    let level = Level::Hard;

    match level {
        Level::Easy => day01::historian_hysteria_easy(),
        Level::Hard => day01::historian_hysteria_hard(),
    }
}
