fn get_value(board: &[&str], y: i32, x: i32) -> Option<char> {
    if y >= 0 && x >= 0 && (y as usize) < board.len() {
        board[y as usize].chars().nth(x as usize)
    } else {
        None
    }
}

fn find_xmas(board: &Vec<&str>, y: i32, x: i32) -> i32 {
    let mut sum = 0;

    if get_value(board, y, x) == Some('X')
        && get_value(board, y - 1, x - 1) == Some('M')
        && get_value(board, y - 2, x - 2) == Some('A')
        && get_value(board, y - 3, x - 3) == Some('S')
    {
        sum += 1
    }

    if get_value(board, y, x) == Some('X')
        && get_value(board, y - 1, x) == Some('M')
        && get_value(board, y - 2, x) == Some('A')
        && get_value(board, y - 3, x) == Some('S')
    {
        sum += 1
    }

    if get_value(board, y, x) == Some('X')
        && get_value(board, y - 1, x + 1) == Some('M')
        && get_value(board, y - 2, x + 2) == Some('A')
        && get_value(board, y - 3, x + 3) == Some('S')
    {
        sum += 1
    }

    if get_value(board, y, x) == Some('X')
        && get_value(board, y, x - 1) == Some('M')
        && get_value(board, y, x - 2) == Some('A')
        && get_value(board, y, x - 3) == Some('S')
    {
        sum += 1
    }

    if get_value(board, y, x) == Some('X')
        && get_value(board, y, x + 1) == Some('M')
        && get_value(board, y, x + 2) == Some('A')
        && get_value(board, y, x + 3) == Some('S')
    {
        sum += 1
    }

    if get_value(board, y, x) == Some('X')
        && get_value(board, y + 1, x - 1) == Some('M')
        && get_value(board, y + 2, x - 2) == Some('A')
        && get_value(board, y + 3, x - 3) == Some('S')
    {
        sum += 1
    }

    if get_value(board, y, x) == Some('X')
        && get_value(board, y + 1, x) == Some('M')
        && get_value(board, y + 2, x) == Some('A')
        && get_value(board, y + 3, x) == Some('S')
    {
        sum += 1
    }

    if get_value(board, y, x) == Some('X')
        && get_value(board, y + 1, x + 1) == Some('M')
        && get_value(board, y + 2, x + 2) == Some('A')
        && get_value(board, y + 3, x + 3) == Some('S')
    {
        sum += 1
    }

    sum
}

pub fn ceres_search_easy() {
    let input = std::fs::read_to_string("../input/day04.txt").unwrap();

    let board = input.split("\n").collect::<Vec<_>>();

    let mut sum = 0;

    for y in 0..board.len() {
        let row = &board[y];
        for x in 0..row.len() {
            sum += find_xmas(&board, y as i32, x as i32);
        }
    }

    println!("{}", sum)
}

fn find_x_mas(board: &Vec<&str>, y: i32, x: i32) -> bool {
    let from_left_top_to_right_bottom = (get_value(board, y - 1, x - 1) == Some('M')
        && get_value(board, y, x) == Some('A')
        && get_value(board, y + 1, x + 1) == Some('S'))
        || (get_value(board, y - 1, x - 1) == Some('S')
            && get_value(board, y, x) == Some('A')
            && get_value(board, y + 1, x + 1) == Some('M'));

    let from_right_top_to_left_bottom = (get_value(board, y - 1, x + 1) == Some('M')
        && get_value(board, y, x) == Some('A')
        && get_value(board, y + 1, x - 1) == Some('S'))
        || (get_value(board, y - 1, x + 1) == Some('S')
            && get_value(board, y, x) == Some('A')
            && get_value(board, y + 1, x - 1) == Some('M'));

    from_left_top_to_right_bottom && from_right_top_to_left_bottom
}

pub fn ceres_search_hard() {
    let input = std::fs::read_to_string("../input/day04.txt").unwrap();

    let board = input.split("\n").collect::<Vec<_>>();

    let mut sum = 0;

    for y in 0..board.len() {
        let row = &board[y];
        for x in 0..row.len() {
            sum += if find_x_mas(&board, y as i32, x as i32) {
                1
            } else {
                0
            };
        }
    }

    println!("{}", sum)
}
