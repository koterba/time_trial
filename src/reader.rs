use crossterm::{
    self,
    event::{
        self, 
        Event,
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
}