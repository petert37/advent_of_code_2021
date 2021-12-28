use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let min_cost = find_min_cost(&input, calc_sum_cost);
    println!("Min cost: {}", min_cost);
    let min_cost = find_min_cost(&input, calc_sum_cost_correct);
    println!("Correct min cost: {}", min_cost);
}

fn find_min_cost(positions: &Vec<i32>, sum_cost_calculator: fn(&Vec<i32>, i32) -> i32) -> i32 {
    let min = positions.iter().min().unwrap();
    let max = positions.iter().max().unwrap();
    return (*min..=*max)
        .map(|target| sum_cost_calculator(positions, target))
        .min()
        .unwrap();
}

fn calc_sum_cost(positions: &Vec<i32>, target: i32) -> i32 {
    positions
        .iter()
        .map(|position| (*position - target).abs())
        .sum()
}

fn calc_sum_cost_correct(positions: &Vec<i32>, target: i32) -> i32 {
    positions
        .iter()
        .map(|position| (*position - target).abs())
        .map(|distance| (distance.pow(2) + distance) / 2)
        .sum()
}
