use std::cmp::{max, min};

use regex::Regex;

fn main() {
    let binding = std::fs::read_to_string("../input").unwrap();
    let board: Vec<_> = binding.lines().collect();

    let symbols_re = Regex::new(r"\=|\%|\@|\/|\+|\&|\-|\$|\*|\#").unwrap();
    let numbers: Vec<_> = board
        .iter()
        .map(|line| symbols_re.replace_all(line, "."))
        .collect();

    let marks = create_marks(board, symbols_re);

    let mut sum = 0;
    for (row, line) in numbers.iter().enumerate() {
        let mut digits: Vec<u32> = vec![];
        let mut was_number = false;
        let mut is_connected = false;
        for (col, ch) in line.as_bytes().iter().enumerate() {
            if (*ch as char) == '.' {
                if was_number {
                    let mut num = 0;
                    for digit in &digits {
                        num = num * 10 + digit;
                    }
                    digits = vec![];
                    was_number = false;
                    if is_connected {
                        sum += num;
                        is_connected = false;
                    }
                }
                continue
            }
            // ch is a number
            was_number = true;
            digits.push((*ch as char).to_string().parse().unwrap());
            if marks[row][col] {
                is_connected = true;
            }
        }
        if was_number {
            let mut num = 0;
            for digit in &digits {
                num = num * 10 + digit;
            }
            if is_connected {
                sum += num;
            }
        }
    }
    dbg!(sum);
}

fn create_marks(board: Vec<&str>, symbols_re: Regex) -> Vec<Vec<bool>> {
    let mut marks: Vec<Vec<bool>> = vec![vec![false; board[0].len()]; board.len()];
    for (row, line) in board.iter().enumerate() {
        for (col, ch) in line.as_bytes().iter().enumerate() {
            if symbols_re.is_match(&(*ch as char).to_string()) {
                for row_offset in max(row - 1, 0)..=min(row + 1, board.len() - 1) {
                    for col_offset in max(col - 1, 0)..=min(col + 1, line.len() - 1) {
                        marks[row_offset][col_offset] = true;
                    }
                }
            }
        }
    }
    marks
}
