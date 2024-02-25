use std::borrow::Cow;
use std::io::Write;

use crate::{ Attr, Colour, Target };

pub struct AnsiTarget <Out: Write> {
	out: Out,
	width: usize,
}

impl <Out: Write> AnsiTarget <Out> {
	pub fn new (out: Out, width: usize) -> Self {
		Self { out, width }
	}
}

impl <'dat, Out: Write> Target <'dat> for AnsiTarget <Out> {

	fn width (& self) -> usize {
		self.width
	}

	fn push_attr (& mut self, attr: Attr) {
		if let Some (Colour { red, green, blue }) = attr.foreground {
			write! (self.out, "\x1b[38;2;{red};{green};{blue}m").unwrap ();
		} else {
			write! (self.out, "\x1b[39m").unwrap ();
		}
		if let Some (Colour { red, green, blue }) = attr.background {
			write! (self.out, "\x1b[48;2;{red};{green};{blue}m").unwrap ();
		} else {
			write! (self.out, "\x1b[49m").unwrap ();
		}
		if attr.bold {
			write! (self.out, "\x1b[1m").unwrap ();
		} else {
			write! (self.out, "\x1b[22m").unwrap ();
		}
	}

	fn push_str (& mut self, text: Cow <'dat, str>) {
		write! (self.out, "{text}").unwrap ();
	}

	fn newline (& mut self) {
		write! (self.out, "\n").unwrap ();
	}

}
