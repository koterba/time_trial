use crate::actions::Move;
use std::time::Instant;
use crate::clear;

pub const PLAYER: char = '♥';
pub const WALL: char = '🮏';
pub const PIECE: char = '^';

#[derive(Clone)]
pub struct Level {
	pub board: Vec<Vec<char>>,
	pub pieces: i32,
	pub level_number: i32,
	pub ending_message: String
}

impl Level {
	pub fn display_board(&self) {
		println!("     Level: {}\n\nRemaining pieces: {}", self.level_number, self.pieces);
		for (index, row) in self.board.iter().enumerate() {
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

	pub fn get_player_location(&self) -> (usize, usize) {
		let mut location = (0 as usize, 0 as usize);
		for (row_index, row) in self.board.iter().enumerate() {
			for (column_index, _column) in row.iter().enumerate() {
				if self.board[row_index][column_index] == PLAYER {
					location = (row_index, column_index);
				}
			}
		}

		location
	}

	pub fn item_next_move(&self, dir: &Move) -> char {
		let (row, column) = self.get_player_location();

		if dir == &Move::Up {
			self.board[row-1][column].clone()
		} else if dir == &Move::Down {
			self.board[row+1][column].clone()
		} else if dir == &Move::Left {
			self.board[row][column-1].clone()
		} else if dir == &Move::Right {
			self.board[row][column+1].clone()
		} else {
			panic!("invalid direction")
		}
	}

	pub fn finish(&self, start_time: Instant) {
	    clear();
	    println!("You passed! Time taken: {:?}\n\n{}", start_time.elapsed(), self.ending_message);
	}
}

pub fn first_level() -> Level {
	let board = vec![
	vec!['╭', '─', '─', '─', '─', '─', '─', '─', '─', '╮'],
	vec!['│', ' ', ' ', '^', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', '^', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', WALL, WALL, WALL, WALL, ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', PLAYER, ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', WALL, WALL, WALL, WALL, WALL, '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['╰', '─', '─', '─', '─', '─', '─', '─', '─', '╯']
	];

	let pieces = 2;
	Level {
		board, 
		pieces, 
		level_number: 1,
		ending_message: String::from("Level 2 will be more difficult, figure out a way to complete it!")
	}
}

pub fn second_level() -> Level {
	let board = vec![
	vec!['╭', '─', '─', '─', '─', '─', '─', '─', '─', '╮'],
	vec!['│', '^', '^', ' ', ' ', ' ', ' ', '^', '^', '│'],
	vec!['│', '^', ' ', WALL, ' ', ' ', ' ', ' ', '^', '│'],
	vec!['│', WALL, WALL, WALL, ' ', ' ', ' ', ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', ' ', ' ', WALL, ' ', ' ', '│'],
	vec!['│', ' ', ' ', ' ', PLAYER, WALL, ' ', ' ', ' ', '│'],
	vec!['│', ' ', WALL, ' ', ' ', ' ', WALL, ' ', ' ', '│'],
	vec!['│', '^', ' ', WALL, ' ', ' ', ' ', ' ', '^', '│'],
	vec!['│', '^', '^', ' ', WALL, ' ', ' ', '^', '^', '│'],
	vec!['╰', '─', '─', '─', '─', '─', '─', '─', '─', '╯']
	];

	let pieces = 12;
	Level {
		board, 
		pieces, 
		level_number: 2,
		ending_message: String::from("Level 3 will consist of a new mechanic, can you figure it out?")
	}
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
	Level {
		board, 
		pieces, 
		level_number: 3,
		ending_message: String::from("You finished it all! Congradulations")
	}
}

