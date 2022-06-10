mod actions;
mod reader;
mod screen;
mod board;
mod menu;

use board::{first_board, second_board, third_board};
use actions::{game_loop, GameAction};
use reader::Reader;
use screen::clear;

use crossterm::{
    self,
    terminal,
    cursor
};

use std::time::Instant;

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
    terminal::enable_raw_mode()?;
    cursor::Hide;

    let mut start_game = menu::display_menu(); // runs the menu cutscene

    let boards = vec![first_board(), second_board(), third_board()];
    let mut current_board = 0;
    let mut board = boards[current_board].clone();

    if start_game {
        println!("Level 1\n\nStart the game by pressing any of the arrow keys.\n\nCollect all the '^' as fast as possible to move on\n");
    }

    let time_since_beginning = Instant::now();

    while start_game {
        let start_time = Instant::now();
        if game_loop(&mut board, start_time, &current_board + 1) == GameAction::Quit {
            break
        } else {
            if current_board != boards.len() - 1 {
                current_board += 1; // one index higher == one level higher: as we are indexing a vector with boards
                board = boards[current_board].clone(); // we clone the new board from the vector and set it as current board
                println!("Press ENTER or MOVE to continue onto Level {}", current_board + 1);
            } else {
                println!("\n\nYou finished! Congratulations.\n\nIt took you: {:?} to speedrun all the levels.", time_since_beginning.elapsed());
                start_game = false;
            }

        }
    }

    Ok(())
}