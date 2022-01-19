use std::fs;

struct Octopus {
    energy_level: u32,
    flashed: bool,
}

struct OctopusGrid {
    octopuses: Vec<Octopus>,
    width: i32,
    height: i32,
}

impl OctopusGrid {
    fn do_step(&mut self) -> i32 {
        self.increase_energy_levels();
        self.do_flashes();
        let flashes = self.count_flashes();
        self.reset_flashed();
        return flashes;
    }

    fn increase_energy_levels(&mut self) {
        for octopus in &mut self.octopuses {
            octopus.energy_level += 1;
        }
    }

    fn do_flashes(&mut self) {
        let mut flashed: bool;
        loop {
            flashed = false;
            for y in 0..self.width {
                for x in 0..self.height {
                    let octopus = self.get_octopus_at_mut(x, y);
                    if let Some(octopus) = octopus {
                        if octopus.energy_level > 9 && !octopus.flashed {
                            octopus.flashed = true;
                            flashed = true;
                            self.increase_neighbor_energy_levels(x, y);
                        }
                    }
                }
            }
            if !flashed {
                break;
            }
        }
    }

    fn get_octopus_at_mut(&mut self, x: i32, y: i32) -> Option<&mut Octopus> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            self.octopuses.get_mut((y * self.width + x) as usize)
        }
    }

    fn get_octopus_at(&self, x: i32, y: i32) -> Option<&Octopus> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            self.octopuses.get((y * self.width + x) as usize)
        }
    }

    fn increase_neighbor_energy_levels(&mut self, x: i32, y: i32) {
        for cy in y - 1..=y + 1 {
            for cx in x - 1..=x + 1 {
                if !(cx == x && cy == y) {
                    let octopus = self.get_octopus_at_mut(cx, cy);
                    if let Some(octopus) = octopus {
                        octopus.energy_level += 1;
                    }
                }
            }
        }
    }

    fn count_flashes(&self) -> i32 {
        self.octopuses
            .iter()
            .filter(|octopus| octopus.flashed)
            .count() as i32
    }

    fn reset_flashed(&mut self) {
        for octopus in &mut self.octopuses {
            if octopus.flashed {
                octopus.energy_level = 0;
                octopus.flashed = false;
            }
        }
    }

    fn print_grid(&self) {
        for y in 0..self.width {
            for x in 0..self.height {
                let octopus = self.get_octopus_at(x, y);
                if let Some(octopus) = octopus {
                    print!("{}", octopus.energy_level)
                }
            }
            println!()
        }
    }
}

fn main() {
    let mut octopus_grid = OctopusGrid {
        octopuses: fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .flat_map(|line| {
                line.chars().map(|char| Octopus {
                    energy_level: char.to_digit(10).unwrap(),
                    flashed: false,
                })
            })
            .collect::<Vec<Octopus>>(),
        width: 10,
        height: 10,
    };
    let mut total_flashes = 0;
    let mut i = 1;
    loop {
        let step_flashes = octopus_grid.do_step();
        total_flashes += step_flashes;
        println!("After step {}:", i);
        octopus_grid.print_grid();
        println!();
        if i == 100 {
            println!("Flashes after 100 steps: {}", total_flashes);
        }
        if step_flashes == 100 {
            println!("Synchronized after {} steps", i);
            break;
        }
        i += 1;
    }
}
