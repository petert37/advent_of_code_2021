use std::fs;

#[derive(Clone)]
struct Cell {
    value: i32,
    marked: bool,
}

#[derive(Clone)]
struct Board {
    cells: Vec<Cell>,
}

impl Board {
    fn from_input(board_input: &Vec<&String>) -> Self {
        let cells = board_input
            .iter()
            .map(|s| Cell {
                value: s.parse::<i32>().unwrap(),
                marked: false,
            })
            .collect::<Vec<Cell>>();
        Board { cells }
    }

    fn mark_cell(&mut self, value: i32) -> bool {
        let cell = self.cells.iter_mut().find(|c| c.value == value);
        match cell {
            Some(c) => {
                c.marked = true;
                true
            }
            None => false,
        }
    }

    fn is_bingo(&self) -> bool {
        for i in 0..5 {
            if self.get_row(i).iter().all(|row| row.marked) {
                return true;
            }
            if self.get_column(i).iter().all(|column| column.marked) {
                return true;
            }
        }
        return false;
    }

    fn get_unmarked_cell_values(&self) -> Vec<&i32> {
        self.cells
            .iter()
            .filter(|cell| !cell.marked)
            .map(|cell| &cell.value)
            .collect::<Vec<&i32>>()
    }

    fn get_cell_index(&self, row: i32, column: i32) -> usize {
        (row * 5 + column % 5) as usize
    }

    fn get_cell(&self, row: i32, column: i32) -> &Cell {
        &self.cells[self.get_cell_index(row, column)]
    }

    fn get_row(&self, row: i32) -> Vec<&Cell> {
        (0..5)
            .map(|column| self.get_cell(row, column))
            .collect::<Vec<&Cell>>()
    }

    fn get_column(&self, column: i32) -> Vec<&Cell> {
        (0..5)
            .map(|row| self.get_cell(row, column))
            .collect::<Vec<&Cell>>()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" ")
                .filter(|s| s.len() > 0)
                .map(|s| String::from(s))
                .collect::<Vec<String>>()
        })
        .filter(|v| v.len() > 0)
        .collect::<Vec<Vec<String>>>();

    let numbers = input[0][0]
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let boards = (1..input.len())
        .step_by(5)
        .map(|i| {
            let board_input = input[i..(i + 5)]
                .iter()
                .flatten()
                .map(|s| s)
                .collect::<Vec<&String>>();
            Board::from_input(&board_input)
        })
        .collect::<Vec<Board>>();

    let mut part_1_boards = boards.clone();
    let (winning_board_index, winning_number) =
        play_numbers_until_win(&mut part_1_boards, &numbers);
    let part_1_result = count_result(&part_1_boards[winning_board_index], winning_number);
    println!("Part 1 result: {}", part_1_result);

    let mut part_2_boards = boards.clone();
    let (winning_board_index, winning_number) =
        play_numbers_until_all_win(&mut part_2_boards, &numbers);
    let part_2_result = count_result(&part_2_boards[winning_board_index], winning_number);
    println!("Part 1 result: {}", part_2_result);
}

fn play_numbers_until_win(boards: &mut Vec<Board>, numbers: &Vec<i32>) -> (usize, i32) {
    for number in numbers {
        for (board_index, board) in (&mut *boards).iter_mut().enumerate() {
            board.mark_cell(*number);
            if board.is_bingo() {
                return (board_index, *number);
            }
        }
    }
    panic!("No winner")
}

fn play_numbers_until_all_win(boards: &mut Vec<Board>, numbers: &Vec<i32>) -> (usize, i32) {
    let board_count = boards.len();
    let mut winner_board_count = 0;
    for number in numbers {
        for (board_index, board) in (&mut *boards).iter_mut().enumerate() {
            if !board.is_bingo() {
                board.mark_cell(*number);
                if board.is_bingo() {
                    winner_board_count += 1;
                    if winner_board_count == board_count {
                        return (board_index, *number);
                    }
                }
            }
        }
    }
    panic!("No winner")
}

fn count_result(winning_board: &Board, winning_number: i32) -> i32 {
    winning_board
        .get_unmarked_cell_values()
        .iter()
        .map(|i| *i)
        .sum::<i32>()
        * winning_number
}
