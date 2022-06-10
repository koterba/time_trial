use crossterm::{
    self,
    event::{
        self, 
        KeyCode, 
        KeyEvent
    }
};

use crate::reader::Reader;

const LOGO: &str = r"       _                            _       _  
   _  (_)                 _        (_)     | | 
 _| |_ _ ____  _____    _| |_  ____ _ _____| | 
(_   _) |    \| ___ |  (_   _)/ ___) (____ | | 
  | |_| | | | | ____|    | |_| |   | / ___ | | 
   \__)_|_|_|_|_____)     \__)_|   |_\_____|\_)

Select an option using the arrow keys:";

pub enum MenuOption {
	Start,
	Quit
}

pub fn display_menu() -> bool {
	let mut current_selection = MenuOption::Start;
	let reader = Reader;

	loop {
		match &current_selection {
			MenuOption::Start => println!("{}\n\n-> Start Game\n   Quit Game", LOGO),
			MenuOption::Quit => println!("{}\n\n   Start Game\n-> Quit Game", LOGO)
		}
		let event = reader.read_key().expect("Could not read key");
		match event {
			KeyEvent {
				code: KeyCode::Up,
				modifiers: event::KeyModifiers::NONE
			} => current_selection = MenuOption::Start,
			KeyEvent {
				code: KeyCode::Down,
				modifiers: event::KeyModifiers::NONE
			} => current_selection = MenuOption::Quit,
			KeyEvent {
				code: KeyCode::Enter,
				modifiers: event::KeyModifiers::NONE
			} => break,
			_ => {}
		}
	}
	match current_selection {
		MenuOption::Start => true,
		MenuOption::Quit => false
	}
}