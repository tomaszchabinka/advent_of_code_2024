use regex::Regex;

pub fn mull_it_over_easy() {
    let input = std::fs::read_to_string("../input/day03.txt").unwrap();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum: i32 = 0;

    for (_, [fst, snd]) in re.captures_iter(&input).map(|c| c.extract()) {
        sum += fst.parse::<i32>().unwrap() * snd.parse::<i32>().unwrap();
    }

    println!("{}", sum);
}

pub fn mull_it_over_hard() {
    let input = "do()".to_string() + &std::fs::read_to_string("../input/day03.txt").unwrap();

    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum: i32 = 0;

    input.split("don't()").for_each(|line| {
        if let Some(pos) = line.find("do()") {
            for (_, [fst, snd]) in re.captures_iter(&line[pos..]).map(|c| c.extract()) {
                sum += fst.parse::<i32>().unwrap() * snd.parse::<i32>().unwrap();
            }
        }
    });

    println!("{}", sum);
}
