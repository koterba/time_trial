use crate::board::{display_board, is_all_collected, collected_amount, get_player_location, completed_board, PLAYER};
use crate::reader::Reader;
use crate::screen::clear;
use std::time::Instant;

use crossterm::{
    self,
    event::{
        self,  
        KeyCode, 
        KeyEvent
    }
};


#[derive(PartialEq)]
pub enum Move {
	Up,
	Down,
	Left,
	Right
}

#[derive(PartialEq)]
pub enum GameAction {
	NextLevel,
	Quit
}

pub fn move_player(board: &mut Vec<Vec<char>>, dir: Move) {
	let (row, column) = get_player_location(&board);

	if dir == Move::Up && row - 1 == 0 {
		return
	} else if dir == Move::Down && row + 1 == 9 {
		return
	} else if dir == Move::Left && column - 1 == 0 {
		return
	} else if dir == Move::Right && column + 1 == 9 {
		return
	}

	match dir {
		Move::Up => {
			board[row][column] = ' ';
			board[row-1][column] = PLAYER;
		},
		Move::Down => {
			board[row][column] = ' ';
			board[row+1][column] = PLAYER;
		},
		Move::Left => {
			board[row][column] = ' ';
			board[row][column-1] = PLAYER;
		},
		Move::Right => {
			board[row][column] = ' ';
			board[row][column+1] = PLAYER;
		},
		
	}
}

pub fn game_loop(board: &mut Vec<Vec<char>>, start_time: Instant, current_level: usize) -> GameAction {
	let reader = Reader;
	loop {
        let event = reader.read_key().expect("Could not read key");
        match event {
            KeyEvent {
                code: KeyCode::Up,
                modifiers: event::KeyModifiers::NONE
            } => move_player(board, Move::Up),
            KeyEvent {
                code: KeyCode::Down,
                modifiers: event::KeyModifiers::NONE
            } => move_player(board, Move::Down),
            KeyEvent {
                code: KeyCode::Left,
                modifiers: event::KeyModifiers::NONE
            } => move_player(board, Move::Left),
            KeyEvent {
                code: KeyCode::Right,
                modifiers: event::KeyModifiers::NONE
            } => move_player(board, Move::Right),
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL
            } => break,
            _ => {}
        }
        clear();
        display_board(&board, collected_amount(&board), current_level);

        if is_all_collected(&board) {
            completed_board(start_time);
            return GameAction::NextLevel
        }
	}

	return GameAction::Quit
}
