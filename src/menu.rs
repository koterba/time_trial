use crossterm::{
    self,
    event::{
        self, 
        Event, 
        KeyCode, 
        KeyEvent
    }
};

use crate::reader::Reader;

enum menu_option {
	Start,
	Quit
}

pub fn display_menu() -> bool {
	let mut current_selection = menu_option::Start;
	let reader = Reader;

	loop {
		println!("Select option using arrow keys:\n");
		match &current_selection {
			menu_option::Start => println!("-> Start Game\n   Quit Game"),
			menu_option::Quit => println!("   Start Game\n-> Quit Game")
		}
		let event = reader.read_key().expect("Could not read key");
		match event {
			KeyEvent {
				code: KeyCode::Up,
				modifiers: event::KeyModifiers::NONE
			} => current_selection = menu_option::Start,
			KeyEvent {
				code: KeyCode::Down,
				modifiers: event::KeyModifiers::NONE
			} => current_selection = menu_option::Quit,
			KeyEvent {
				code: KeyCode::Enter,
				modifiers: event::KeyModifiers::NONE
			} => break,
			_ => {}
		}
	}
	match current_selection {
		menu_option::Start => true,
		menu_option::Quit => false
	}
}