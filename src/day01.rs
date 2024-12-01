pub fn historian_hysteria_easy() {
    // read the input from the file, input consists of multiple lines, where every line gives two number. First number should be read to first array, second number should be read to second array
    let input = std::fs::read_to_string("../input/day01.txt").unwrap();
    let mut first_array: Vec<u32> = Vec::new();
    let mut second_array: Vec<u32> = Vec::new();
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        first_array.push(numbers[0]);
        second_array.push(numbers[1]);
    }

    // sort the first array
    first_array.sort();
    // sort the second array
    second_array.sort();

    // loop through the first array and find the corresponding number in the second array
    let mut diff = 0;
    for i in 0..first_array.len() {
        diff += first_array[i].abs_diff(second_array[i]);
    }

    println!("{}", diff);
}

pub fn historian_hysteria_hard() {
    // read the input from the file, input consists of multiple lines, where every line gives two number. First number should be read to first array, second number should be read to second array
    let input = std::fs::read_to_string("../input/day01.txt").unwrap();
    let mut first_array: Vec<u32> = Vec::new();
    let mut second_array: Vec<u32> = Vec::new();
    for line in input.lines() {
        let numbers: Vec<u32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        first_array.push(numbers[0]);
        second_array.push(numbers[1]);
    }

    // count number of timest every element appears in the second list
    let mut count_map: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
    for i in 0..second_array.len() {
        *count_map.entry(second_array[i]).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for i in first_array {
        similarity_score += i * count_map.get(&i).unwrap_or(&0);
    }

    println!("{}", similarity_score);
}
