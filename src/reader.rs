use crossterm::{
    self,
    event::{
        self, 
        Event, 
        KeyCode, 
        KeyEvent
    }
};

use crate::clear;

pub struct Reader;

impl Reader {
	pub fn read_key(&self) -> crossterm::Result<KeyEvent> {
		loop {
			if let Event::Key(event) = event::read()? {
				clear();
				return Ok(event);
			}
    	}
	}

	

	pub fn proccess_key(&self) -> crossterm::Result<bool> {
		let event = self.read_key()?;
		match event {
			KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: event::KeyModifiers::CONTROL
			} => return Ok(false),
			_ => {}
		}
		Ok(true)
	}

	pub fn run(&self) -> crossterm::Result<bool> {
		self.proccess_key()
	}
}