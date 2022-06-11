mod actions;
mod reader;
mod screen;
mod level;
mod menu;

use level::{first_level, second_level, third_level};
use actions::{game_loop, GameAction};
use screen::clear;

use crossterm::{
    self,
    terminal
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
    // START: TERMINAL SETUP
    clear();
    let _cleaner = CleanUp; // instance of cleanup, so it drops after the block ends, disabling raw mode
    terminal::enable_raw_mode()?;
    // END

    // runs the menu cutscene
    let mut start_game = menu::display_menu();

    // LEVEL SETUP
    let levels = vec![first_level(), second_level(), third_level()];
    let mut current_level = 0;
    let mut level = levels[current_level].clone();

    if start_game {
        println!("Level 1\n\nStart the game by pressing any of the arrow keys.\n\nCollect all the '^' as fast as possible to move on\n");
    }

    // keeps count of the time taken since the very start till the end of the game
    let time_since_beginning = Instant::now();

    while start_game {
        let start_time = Instant::now();
        if game_loop(&mut level, start_time) == GameAction::Quit {
            break
        } else {
            if current_level != levels.len() - 1 { // if this is not the last level
                current_level += 1; // one index higher == one level higher: as we are indexing a vector with levels
                level = levels[current_level].clone(); // we clone the new level from the vector and set it as current level
                println!("\nPress ENTER or MOVE to continue onto Level {}", current_level + 1);
            } else {
                println!("It took you: {:?} to speedrun all the levels.\n", time_since_beginning.elapsed());
                start_game = false;
            }

        }
    }
    Ok(())
}