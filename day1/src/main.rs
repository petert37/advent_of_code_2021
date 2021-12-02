use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    println!("Part 1 result: {}", count_increases(&input));

    let windowed: Vec<i32> = input.windows(3).map(|x| x.iter().sum()).collect();
    println!("Part 2 result: {}", count_increases(&windowed));
}

fn count_increases(input: &Vec<i32>) -> i32 {
    let mut result = 0;
    for (i, elem) in input.iter().enumerate() {
        if i > 0 && input[i - 1] < *elem {
            result += 1;
        }
    }
    return result;
}
