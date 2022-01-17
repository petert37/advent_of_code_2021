use std::fs;

fn main() {
    let lines = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let mut score_part_1 = 0;
    let mut unfinished_stacks: Vec<Vec<char>> = vec![];
    for line in &lines {
        let (stack, char) = get_line_stack(line);
        if let Some(char) = char {
            score_part_1 += get_char_score_part_1(&char);
        } else {
            unfinished_stacks.push(stack);
        }
    }
    println!("Score: {}", score_part_1);

    let mut scores_part_2 = unfinished_stacks
        .iter()
        .map(get_unfinished_line_score)
        .collect::<Vec<i64>>();
    scores_part_2.sort();
    let score_part_2 = scores_part_2.get(scores_part_2.len() / 2).unwrap();
    println!("Score: {}", score_part_2);
}

fn get_line_stack(line: &str) -> (Vec<char>, Option<char>) {
    let mut stack: Vec<char> = vec![];
    for char in line.chars() {
        if is_chunk_opener(&char) {
            stack.push(char)
        } else if is_chunk_closer(&char) && !stack.is_empty() {
            let last_opener = stack.pop().unwrap();
            if is_chunk_closer(&last_opener) || get_chunk_closer_for(&last_opener) != char {
                stack.push(last_opener);
                return (stack, Some(char));
            }
        }
    }
    return (stack, None);
}

fn get_unfinished_line_score(line: &Vec<char>) -> i64 {
    let mut score: i64 = 0;
    for c in line.iter().rev() {
        score = score * 5 + get_char_score_part_2(&get_chunk_closer_for(c)) as i64;
    }
    return score;
}

fn is_chunk_opener(c: &char) -> bool {
    match c {
        '(' => true,
        '[' => true,
        '{' => true,
        '<' => true,
        _ => false,
    }
}

fn is_chunk_closer(c: &char) -> bool {
    match c {
        ')' => true,
        ']' => true,
        '}' => true,
        '>' => true,
        _ => false,
    }
}

fn get_chunk_closer_for(c: &char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Not a chunk opener: {}", c),
    }
}

fn get_char_score_part_1(c: &char) -> i32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_char_score_part_2(c: &char) -> i32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}
