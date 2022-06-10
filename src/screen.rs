use crossterm::{
	terminal::{
		self,
		ClearType
	},
	execute,
	cursor
};

use std::io::stdout;

pub fn clear() {
	execute!(
		stdout(),
		terminal::Clear(ClearType::All)
	).expect("Could not clear");
	execute!(
		stdout(),
		cursor::MoveTo(0, 0)
	).expect("Could not move cursor to 0,0");
	execute!(
		stdout(),
		cursor::Hide
	).expect("Could not move cursor to 0,0")

}