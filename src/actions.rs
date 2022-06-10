use crate::level::{display_board, get_player_location, completed_board, PLAYER, Level};
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

pub fn move_player(level: &mut Level, dir: Move) {
	let (row, column) = get_player_location(level);

	if dir == Move::Up && row - 1 == 0 {
		return
	} else if dir == Move::Down && row + 1 == 9 {
		return
	} else if dir == Move::Left && column - 1 == 0 {
		return
	} else if dir == Move::Right && column + 1 == 9 {
		return
	}

	if dir == Move::Up && level.board[row-1][column] == '^' {
		level.pieces -= 1
	} else if dir == Move::Down && level.board[row+1][column] == '^' {
		level.pieces -= 1
	} else if dir == Move::Left && level.board[row][column-1] == '^' {
		level.pieces -= 1
	} else if dir == Move::Right && level.board[row][column+1] == '^' {
		level.pieces -= 1
	}

	match dir {
		Move::Up => {
			level.board[row][column] = ' ';
			level.board[row-1][column] = PLAYER;
		},
		Move::Down => {
			level.board[row][column] = ' ';
			level.board[row+1][column] = PLAYER;
		},
		Move::Left => {
			level.board[row][column] = ' ';
			level.board[row][column-1] = PLAYER;
		},
		Move::Right => {
			level.board[row][column] = ' ';
			level.board[row][column+1] = PLAYER;
		},
		
	}
}

pub fn game_loop(level: &mut Level, start_time: Instant) -> GameAction {
	let reader = Reader;
	loop {
        let event = reader.read_key().expect("Could not read key");
        match event {
            KeyEvent {
                code: KeyCode::Up,
                modifiers: event::KeyModifiers::NONE
            } => move_player(level, Move::Up),
            KeyEvent {
                code: KeyCode::Down,
                modifiers: event::KeyModifiers::NONE
            } => move_player(level, Move::Down),
            KeyEvent {
                code: KeyCode::Left,
                modifiers: event::KeyModifiers::NONE
            } => move_player(level, Move::Left),
            KeyEvent {
                code: KeyCode::Right,
                modifiers: event::KeyModifiers::NONE
            } => move_player(level, Move::Right),
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL
            } => break,
            _ => {}
        }
        clear();
        display_board(level);

      //if level is completed
        if level.pieces == 0 {
            completed_board(start_time);
            return GameAction::NextLevel
        }
	}

	return GameAction::Quit
}
