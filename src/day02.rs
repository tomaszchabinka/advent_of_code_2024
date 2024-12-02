fn is_report_safe(levels: &[i32]) -> bool {
    let is_growing = levels[1] > levels[0];

    let mut is_report_safe = true;

    for idx in 1..levels.len() {
        let diff = (levels[idx] - levels[idx - 1]) * (if is_growing { 1 } else { -1 });
        if !(1..=3).contains(&diff) {
            is_report_safe = false;
        }
    }

    is_report_safe
}

pub fn rednosed_reports_easy() {
    let input = std::fs::read_to_string("../input/day02.txt").unwrap();

    let mut safe_reports = 0;

    for report in input.lines() {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if is_report_safe(&levels[..]) {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);
}

pub fn rednosed_reports_hard() {
    let input = std::fs::read_to_string("../input/day02.txt").unwrap();

    let mut safe_reports = 0;

    for report in input.lines() {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut safe = false;

        for idx in 0..levels.len() {
            let beginning = &levels[0..idx];
            let end = &levels[idx + 1..levels.len()];
            let slice = &[beginning, end].concat()[..];
            safe = safe || is_report_safe(slice);
        }

        if safe {
            safe_reports += 1;
        }
    }

    println!("{}", safe_reports);
}
