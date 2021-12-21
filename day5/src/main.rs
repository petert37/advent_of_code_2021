use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

struct Line {
    start: Coordinate,
    end: Coordinate,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal_or_vertical(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    fn get_points(&self) -> Vec<Coordinate> {
        if self.is_horizontal() {
            if self.start.x <= self.end.x {
                self.start.x..=self.end.x
            } else {
                self.end.x..=self.start.x
            }
            .map(|x| Coordinate { x, y: self.start.y })
            .collect::<Vec<Coordinate>>()
        } else if self.is_vertical() {
            if self.start.y <= self.end.y {
                self.start.y..=self.end.y
            } else {
                self.end.y..=self.start.y
            }
            .map(|y| Coordinate { x: self.start.x, y })
            .collect::<Vec<Coordinate>>()
        } else {
            let x_dir = if self.start.x <= self.end.x { 1 } else { -1 };
            let y_dir = if self.start.y <= self.end.y { 1 } else { -1 };
            let len = (self.start.x - self.end.x).abs();
            (0..=len)
                .map(|i| Coordinate {
                    x: self.start.x + i * x_dir,
                    y: self.start.y + i * y_dir,
                })
                .collect::<Vec<Coordinate>>()
        }
    }
}

fn main() {
    let lines = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let line_numbers = line
                .split(" -> ")
                .map(|line_part| line_part.split(",").map(|num| num.parse::<i32>().unwrap()))
                .flatten()
                .collect::<Vec<i32>>();
            Line {
                start: Coordinate {
                    x: line_numbers[0],
                    y: line_numbers[1],
                },
                end: Coordinate {
                    x: line_numbers[2],
                    y: line_numbers[3],
                },
            }
        })
        .collect::<Vec<Line>>();

    let mut points: HashMap<Coordinate, i32> = HashMap::new();

    let horizontal_or_vertical_lines = lines
        .iter()
        .filter(|line| line.is_horizontal_or_vertical())
        .collect::<Vec<&Line>>();
    add_points(&horizontal_or_vertical_lines, &mut points);
    println!(
        "Dangerous horizontal and vertical point count: {}",
        dangerous_point_count(&points)
    );

    let diagonal_lines = lines
        .iter()
        .filter(|line| !line.is_horizontal_or_vertical())
        .collect::<Vec<&Line>>();
    add_points(&diagonal_lines, &mut points);
    println!(
        "Dangerous total point count: {}",
        dangerous_point_count(&points)
    );
}

fn add_points(lines: &Vec<&Line>, points: &mut HashMap<Coordinate, i32>) {
    for line in lines {
        for point in line.get_points() {
            let new_value = match points.get(&point) {
                Some(&value) => value + 1,
                None => 1,
            };
            points.insert(point, new_value);
        }
    }
}

fn dangerous_point_count(points: &HashMap<Coordinate, i32>) -> usize {
    points.iter().filter(|value| *(*value).1 > 1).count()
}
