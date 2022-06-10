use crate::board;

#[derive(PartialEq)]
pub enum Move {
	Up,
	Down,
	Left,
	Right
}

pub fn move_player(board: &mut Vec<Vec<char>>, dir: Move) {
	let (row, column) = board::get_player_location(&board);

	// make sure that player cannot go out of bounds (non valid index)
	if dir == Move::Up && row - 1 == 0 {
		return
	} else if dir == Move::Down && row + 1 == 9 {
		return
	} else if dir == Move::Left && column - 1 == 0 {
		return
	} else if dir == Move::Right && column + 1 == 9 {
		return
	}

	// replace player with empty space, replace empty space in correct
	// direction with the player
	match dir {
		Move::Up => {
			board[row][column] = ' ';
			board[row-1][column] = '*';
		},
		Move::Down => {
			board[row][column] = ' ';
			board[row+1][column] = '*';
		},
		Move::Left => {
			board[row][column] = ' ';
			board[row][column-1] = '*';
		},
		Move::Right => {
			board[row][column] = ' ';
			board[row][column+1] = '*';
		},
		
	}
}
