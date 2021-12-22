use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut fishes = HashMap::new();

    for i in 0..=8 {
        let count = input.iter().filter(|n| **n == i).count() as i64;
        fishes.insert(i, count);
    }

    simulate_days(&mut fishes, 80);
    let fish_count: i64 = fishes.values().sum();
    println!("Fish count after 80 days: {}", fish_count);

    simulate_days(&mut fishes, 256 - 80);
    let fish_count: i64 = fishes.values().sum();
    println!("Fish count after 256 days: {}", fish_count);
}

fn simulate_days(fishes: &mut HashMap<i32, i64>, days: i32) {
    for _ in 0..days {
        let mut change: HashMap<i32, i64> = HashMap::new();
        for i in 0..=8 {
            change.insert(i, 0);
        }
        for i in 0..=8 {
            let fish_count = *fishes.get(&i).unwrap_or(&0);
            if i == 0 {
                change.entry(0).and_modify(|v| *v -= fish_count);
                change.entry(8).and_modify(|v| *v += fish_count);
                change.entry(6).and_modify(|v| *v += fish_count);
            } else {
                change.entry(i).and_modify(|v| *v -= fish_count);
                change.entry(i - 1).and_modify(|v| *v += fish_count);
            }
        }
        for i in 0..=8 {
            fishes
                .entry(i)
                .and_modify(|v| *v += change.get(&i).unwrap_or(&0));
        }
    }
}
