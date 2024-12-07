use std::collections::HashMap;

pub fn print_queue_easy() {
    let input = std::fs::read_to_string("../input/day05.txt").unwrap();

    let mut is_reading_rules = true;

    let mut rulebook = HashMap::new();

    let mut middle_page_number_sum = 0;

    input.split("\n").for_each(|line| {
        if is_reading_rules {
            if line.is_empty() {
                is_reading_rules = false;
            } else {
                let rule: Vec<&str> = line.split("|").collect();
                let before = rule[0].parse::<i32>().unwrap();
                let after: i32 = rule[1].parse::<i32>().unwrap();

                rulebook.entry(after).or_insert_with(Vec::new).push(before);
            }
        } else {
            if line.is_empty() {
                return;
            }

            let update: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

            let mut old_pages: Vec<&i32> = Vec::new();

            let mut is_rule_broken = false;

            for new_page in &update {
                for old_page in &old_pages {
                    if rulebook.contains_key(old_page) && rulebook[old_page].contains(new_page) {
                        is_rule_broken = true;
                        break;
                    }
                }
                old_pages.push(new_page);
            }

            if !is_rule_broken {
                let idx = update.len() / 2;
                middle_page_number_sum += update[idx];
            }
        }
    });

    println!("{}", middle_page_number_sum);
}

pub fn print_queue_hard() {
    let input = std::fs::read_to_string("../input/day05.txt").unwrap();

    let mut is_reading_rules = true;

    let mut rulebook = HashMap::new();

    let mut middle_page_number_sum = 0;

    input.split("\n").for_each(|line| {
        if is_reading_rules {
            if line.is_empty() {
                is_reading_rules = false;
            } else {
                let rule: Vec<&str> = line.split("|").collect();
                let before = rule[0].parse::<i32>().unwrap();
                let after: i32 = rule[1].parse::<i32>().unwrap();

                rulebook.entry(after).or_insert_with(Vec::new).push(before);
            }
        } else {
            if line.is_empty() {
                return;
            }

            let update: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

            let mut old_pages: Vec<&i32> = Vec::new();

            let mut is_rule_broken = false;

            for new_page in &update {
                for idx in 0..old_pages.len() {
                    if !old_pages.contains(&new_page)
                        && rulebook.contains_key(old_pages[idx])
                        && rulebook[old_pages[idx]].contains(new_page)
                    {
                        old_pages.insert(idx, new_page);
                        is_rule_broken = true;
                        break;
                    }
                }
                if !old_pages.contains(&new_page) {
                    old_pages.push(new_page);
                }
            }

            if is_rule_broken {
                println!("{:?}", old_pages);
                let idx = old_pages.len() / 2;
                middle_page_number_sum += old_pages[idx];
            }
        }
    });

    println!("{}", middle_page_number_sum);
}
