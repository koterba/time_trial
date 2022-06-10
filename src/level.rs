use std::time::Instant;
use crate::clear;

pub const PLAYER: char = '♥';

#[derive(Clone)]
pub struct Level {
	pub board: Vec<Vec<char>>,
	pub pieces: i32,
	pub level_number: i32
}

pub fn first_level() -> Level {
	let board = vec![
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
	];

	let pieces = 2;
	Level {board, pieces, level_number: 1}
}

pub fn second_level() -> Level {
	let board = vec![
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
	];

	let pieces = 12;
	Level {board, pieces, level_number: 2}
}

pub fn third_level() -> Level {
	let board = vec![
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
	];

	let pieces = 22;
	Level {board, pieces, level_number: 3}
}

pub fn display_board(level: &mut Level) {
	println!("     Level: {}\n\nRemaining pieces: {}", level.level_number, level.pieces);
	for (index, row) in level.board.iter().enumerate() {
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

pub fn get_player_location(level: &Level) -> (usize, usize) {
	let mut location = (0 as usize, 0 as usize);
	for (row_index, row) in level.board.iter().enumerate() {
		for (column_index, _column) in row.iter().enumerate() {
			if level.board[row_index][column_index] == PLAYER {
				location = (row_index, column_index);
			}
		}
	}

	location
}

pub fn completed_board(start_time: Instant) {
    clear();
    println!("You passed!\n\nTime taken: {:?}", start_time.elapsed());
}
