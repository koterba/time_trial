use std::time::Instant;
use crate::clear;

pub const PLAYER: char = '♥';

// initial player pos: [5][4]
pub fn first_board() -> Vec<Vec<char>> {
	vec![
	vec!['╭', '─', '─', '─', '─', '─', '─', '─', '─', '╮'],
	vec!['│', ' ', ' ', '^', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', '^', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', PLAYER, ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['╰', '─', '─', '─', '─', '─', '─', '─', '─', '╯']
	]
}

pub fn second_board() -> Vec<Vec<char>> {
	vec![
	vec!['╭', '─', '─', '─', '─', '─', '─', '─', '─', '╮'],
	vec!['│', '^', '^', ' ', ' ', ' ', ' ', '^', '^', '│'],
	vec!['│', '^', ' ', ' ', ' ', ' ', ' ', ' ', '^', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', PLAYER, ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', '^', ' ', ' ', ' ', ' ', ' ', ' ', '^', '│'],
	vec!['│', '^', '^', ' ', ' ', ' ', ' ', '^', '^', '│'],
	vec!['╰', '─', '─', '─', '─', '─', '─', '─', '─', '╯']
	]
}

pub fn third_board() -> Vec<Vec<char>> {
	vec![
	vec!['╭', '─', '─', '─', '─', '─', '─', '─', '─', '╮'],
	vec!['│', ' ', ' ', '^', ' ', ' ', '^', ' ', ' ', '│'],
	vec!['│', '^', '^', ' ', '^', ' ', '^', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '^', '│'],
	vec!['│', '^', ' ', '^', ' ', '^', ' ', ' ', '^', '│'],
	vec!['│', ' ', ' ', ' ', PLAYER, ' ', ' ', '^', ' ', '│'],
	vec!['│', ' ', '^', ' ', ' ', ' ', '^', ' ', '^', '│'],
	vec!['│', '^', '^', '^', ' ', '^', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', '^', '^', ' ', '^', ' ', '│'],
	vec!['╰', '─', '─', '─', '─', '─', '─', '─', '─', '╯']
	]
}

pub fn display_board(board: &Vec<Vec<char>>, pieces_left: i32, current_level: usize) {
	println!("     Level: {}\n\nRemaining pieces: {}", current_level, pieces_left);
	for (index, row) in board.iter().enumerate() {
		for (elm_index, character) in row.iter().enumerate() {
			if (index == 0 || index == 9) && elm_index != 9 {
				print!("{character}─");
			} else {
				print!("{character} ");
			}
		}
		println!("");
	}
}

pub fn get_player_location(board: &Vec<Vec<char>>) -> (usize, usize) {
	let mut location = (0 as usize, 0 as usize);
	for (row_index, row) in board.iter().enumerate() {
		for (column_index, _column) in row.iter().enumerate() {
			if board[row_index][column_index] == PLAYER {
				location = (row_index, column_index);
			}
		}
	}

	location
}

pub fn is_all_collected(board: &Vec<Vec<char>>) -> bool {
	let mut piece_count = 0;

	for row in board {
		for column in row {
			if column == &'^' {
				piece_count += 1;
			}
		}
	}

	if piece_count != 0 {
		false
	} else {
		true
	}
}

pub fn collected_amount(board: &Vec<Vec<char>>) -> i32 {
	let mut piece_count = 0;

	for row in board {
		for column in row {
			if column == &'^' {
				piece_count += 1;
			}
		}
	}

	piece_count

}

pub fn completed_board(start_time: Instant) {
    clear();
    println!("You passed!\n\nTime taken: {:?}", start_time.elapsed());
}
