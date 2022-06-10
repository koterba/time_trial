mod reader;
mod screen;
mod menu;
mod board;
mod actions;

use reader::Reader;
use screen::clear;
use board::{get_board, display_board, get_player_location, is_all_collected};
use actions::{move_player, Move};

use std::time::Instant;

use crossterm::{
    self,
    event::{
        self, 
        Event, 
        KeyCode, 
        KeyEvent
    },
    terminal
};

// Implementing drop for terminal raw mode to make sure that it gets disabled
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode()
            .expect("Could not disable raw mode");
    }
}

fn main() -> crossterm::Result<()> {
    clear();
    let _cleaner = CleanUp; // instance of cleanup, so it drops after the block ends, disabling raw mode
    let reader = Reader; // for reading keys from terminal
    terminal::enable_raw_mode()?;

    let start_game = menu::display_menu(); // run menu scene
    let mut board = get_board();

    println!("Start the game by pressing any of the arrow keys.\nCollect all the '^' as fast as possible to win");
    let event = reader.read_key()?; // read_key will wait, which is good as the timer will start only once the user presses a key
    display_board(&board);

    let start_time = Instant::now(); // start timer once the user presses a key

    while start_game {
        let event = reader.read_key()?;
        // all keys for moving the player
        match event {
            KeyEvent {
                code: KeyCode::Up,
                modifiers: event::KeyModifiers::NONE
            } => move_player(&mut board, Move::Up),
            KeyEvent {
                code: KeyCode::Down,
                modifiers: event::KeyModifiers::NONE
            } => move_player(&mut board, Move::Down),
            KeyEvent {
                code: KeyCode::Left,
                modifiers: event::KeyModifiers::NONE
            } => move_player(&mut board, Move::Left),
            KeyEvent {
                code: KeyCode::Right,
                modifiers: event::KeyModifiers::NONE
            } => move_player(&mut board, Move::Right),
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL
            } => break,
            _ => {}
        }
        clear();
        display_board(&board);

        // if no more pieces are left, finish the game
        if is_all_collected(&board) {
            break
        }
    }

    clear();
    println!("It took you: {:?} Seconds to finish this game.", start_time.elapsed());

    Ok(())
}
