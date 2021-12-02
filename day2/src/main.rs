use std::fs;

struct Command {
    name: String,
    value: i32,
}

impl Command {
    fn apply(&self, horizontal_position: &mut i32, depth: &mut i32) {
        match self.name.as_str() {
            "forward" => *horizontal_position += self.value,
            "down" => *depth += self.value,
            "up" => *depth -= self.value,
            _ => panic!("Invalid command: {}", self.name),
        }
    }

    fn apply_correct(&self, horizontal_position: &mut i32, depth: &mut i32, aim: &mut i32) {
        match self.name.as_str() {
            "forward" => {
                *horizontal_position += self.value;
                *depth += *aim * self.value;
            }
            "down" => *aim += self.value,
            "up" => *aim -= self.value,
            _ => panic!("Invalid command: {}", self.name),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let s = line.split(" ").collect::<Vec<&str>>();
            Command {
                name: String::from(s[0]),
                value: s[1].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Command>>();

    let mut horizontal_position = 0;
    let mut depth = 0;

    for command in &input {
        command.apply(&mut horizontal_position, &mut depth)
    }

    println!(
        "Horizontal position: {}, depth: {}, result: {}",
        horizontal_position,
        depth,
        horizontal_position * depth
    );

    horizontal_position = 0;
    depth = 0;
    let mut aim = 0;

    for command in &input {
        command.apply_correct(&mut horizontal_position, &mut depth, &mut aim)
    }

    println!(
        "Horizontal position: {}, depth: {}, aim: {}, result: {}",
        horizontal_position,
        depth,
        aim,
        horizontal_position * depth
    );
}
