use std::io::{stdout, Write};

// initial player pos: [5][4]
pub fn get_board() -> Vec<Vec<char>> {
	vec![
	vec!['-', '-', '-', '-', '-', '-', '-', '-', '-', '-'],
	vec!['|', ' ', ' ', '^', ' ', ' ', ' ', ' ', '^', '|'],
	vec!['|', '^', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|'],
	vec!['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|'],
	vec!['|', ' ', ' ', ' ', ' ', ' ', '^', ' ', ' ', '|'],
	vec!['|', '^', ' ', ' ', '*', ' ', ' ', ' ', '^', '|'],
	vec!['|', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '|'],
	vec!['|', ' ', ' ', '^', ' ', ' ', ' ', ' ', ' ', '|'],
	vec!['|', '^', ' ', ' ', ' ', '^', ' ', ' ', '^', '|'],
	vec!['-', '-', '-', '-', '-', '-', '-', '-', '-', '-']
	]
}

pub fn display_board(board: &Vec<Vec<char>>) {
	for row in board {
		for character in row {
			print!("{character} ");
		}
		println!("");
	}
}

pub fn get_player_location(board: &Vec<Vec<char>>) -> (usize, usize) {
	let mut location = (0 as usize, 0 as usize);
	for (row_index, row) in board.iter().enumerate() {
		for (column_index, column) in row.iter().enumerate() {
			if board[row_index][column_index] == '*' {
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
