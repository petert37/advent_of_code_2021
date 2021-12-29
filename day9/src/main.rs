use std::fs;

struct Map {
    points: Vec<Vec<Point>>,
    width: i32,
    height: i32,
}

#[derive(Copy, Clone)]
struct Point {
    value: i32,
    x: i32,
    y: i32,
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Map {
    fn from_str(input: &str) -> Self {
        let values = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| Point {
                        value: c.to_digit(10).unwrap() as i32,
                        x: x as i32,
                        y: y as i32,
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>();
        Map {
            width: values.get(0).unwrap().len() as i32,
            height: values.len() as i32,
            points: values,
        }
    }

    fn get_point_at(&self, x: i32, y: i32) -> Option<&Point> {
        if let Some(py) = self.points.get(y as usize) {
            if let Some(px) = py.get(x as usize) {
                Some(px)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_neighbors(&self, x: i32, y: i32) -> Vec<&Point> {
        vec![(x + 1, y), (x, y - 1), (x - 1, y), (x, y + 1)]
            .iter()
            .filter_map(|(xx, yy)| self.get_point_at(*xx, *yy))
            .collect()
    }

    fn get_low_points(&self) -> Vec<&Point> {
        let mut result = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                let point = self.get_point_at(x, y);
                if let Some(point) = point {
                    let neighbors = self.get_neighbors(x, y);
                    if neighbors
                        .iter()
                        .all(|neighbor| neighbor.value > point.value)
                    {
                        result.push(point);
                    }
                }
            }
        }
        return result;
    }

    fn calculate_basins(&self) -> Vec<Vec<Point>> {
        return self
            .get_low_points()
            .iter()
            .map(|low_point| {
                let mut basin: Vec<Point> = vec![**low_point];
                self.add_neighbors_to_basin(&mut basin, low_point.x, low_point.y);
                basin
            })
            .collect::<Vec<Vec<Point>>>();
    }

    fn add_neighbors_to_basin(&self, basin: &mut Vec<Point>, x: i32, y: i32) {
        let unvisited_basin_neighbors = self
            .get_neighbors(x, y)
            .iter()
            .filter(|point| point.value < 9 && !basin.contains(point))
            .map(|point| *point)
            .collect::<Vec<&Point>>();
        if unvisited_basin_neighbors.is_empty() {
            return;
        }
        for point in &unvisited_basin_neighbors {
            basin.push(**point);
        }
        for point in &unvisited_basin_neighbors {
            self.add_neighbors_to_basin(basin, point.x, point.y);
        }
    }
}

fn main() {
    let map = Map::from_str(&fs::read_to_string("input.txt").unwrap());

    let low_points = map.get_low_points();
    let sum: i32 = low_points.iter().map(|point| point.value + 1).sum();
    println!("Sum risk level: {}", sum);

    let basins = map.calculate_basins();
    let mut basin_sizes = basins
        .iter()
        .map(|basin| basin.len() as i32)
        .collect::<Vec<i32>>();
    basin_sizes.sort();
    basin_sizes.reverse();
    let biggest_basin_size_product: i32 = basin_sizes[0..3].iter().product();
    println!("Biggest basins product: {}", biggest_basin_size_product);
}
