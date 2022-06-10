use crossterm::{
	terminal::{
		self,
		ClearType
	},
	execute,
	cursor
};

use std::io::stdout;

// clear screen, move cursor to the top left
pub fn clear() -> crossterm::Result<()> {
	execute!(
		stdout(),
		terminal::Clear(ClearType::All)
	);
	execute!(
		stdout(),
		cursor::MoveTo(0, 0)
	)

}
