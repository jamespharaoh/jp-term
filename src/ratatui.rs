use crossterm::event as ct_event;
use crossterm::terminal as ct_term;
use ratatui::backend as rat_back;
use ratatui::style as rat_style;
use ratatui::text::{ Line, Span, Text };
use std::borrow::Cow;
use std::io::Write;
use std::mem;

use crate::{ Attr, Colour, Target };

pub struct RatTerm <Out: Write> {
	term: Option <ratatui::Terminal <rat_back::CrosstermBackend <Out>>>,
}

impl <Out: Write> RatTerm <Out> {

	pub fn new (mut out: Out, title: & 'static str) -> anyhow::Result <Self> {
		ct_term::enable_raw_mode () ?;
		crossterm::execute! (
			out,
			ct_term::EnterAlternateScreen,
			ct_term::Clear (ct_term::ClearType::All),
			ct_event::EnableMouseCapture,
			ct_term::SetTitle (title)) ?;
		let term_backend = rat_back::CrosstermBackend::new (out);
		let term = ratatui::Terminal::new (term_backend) ?;
		Ok (Self {
			term: Some (term),
		})
	}

	pub fn close (mut self) -> anyhow::Result <()> {
		self.close_real ()
	}

	fn close_real (& mut self) -> anyhow::Result <()> {
		let Some (mut term) = self.term.take () else { return Ok (()) };
		ct_term::disable_raw_mode () ?;
		crossterm::execute! (
			term.backend_mut (),
			ct_term::LeaveAlternateScreen,
			ct_event::DisableMouseCapture) ?;
		term.show_cursor () ?;
		Ok (())
	}

	pub fn term_mut (& mut self) -> & mut ratatui::Terminal <rat_back::CrosstermBackend <Out>> {
		self.term.as_mut ().unwrap ()
	}

}

impl <Out: Write> Drop for RatTerm <Out> {
	fn drop (& mut self) {
		let _ = self.close_real ();
	}
}

impl From <Attr> for rat_style::Style {

	fn from (attr: Attr) -> Self {
		let mut style = Self::default ();
		if let Some (foreground) = attr.foreground {
			style = style.fg (foreground.into ());
		} else {
			style = style.fg (rat_style::Color::Reset);
		}
		if let Some (background) = attr.background {
			style = style.bg (background.into ());
		} else {
			style = style.bg (rat_style::Color::Reset);
		}
		if attr.bold {
			style = style.add_modifier (rat_style::Modifier::BOLD);
		} else {
			style = style.remove_modifier (rat_style::Modifier::BOLD);
		}
		style
	}

}

impl From <Colour> for rat_style::Color {

	fn from (colour: Colour) -> Self {
		Self::Rgb (colour.red, colour.green, colour.blue)
	}

}

pub struct TextTargetBorrow <'tar> {
	width: usize,
	lines: Vec <Line <'tar>>,
	spans: Vec <Span <'tar>>,
	attr: Attr,
}

impl <'tar> TextTargetBorrow <'tar> {

	pub fn new (width: usize) -> Self {
		Self {
			width,
			lines: Vec::new (),
			spans: Vec::new (),
			attr: Attr::default (),
		}
	}

	pub fn build (self) -> Text <'tar> {
		let Self { mut lines, spans, .. } = self;
		if ! spans.is_empty () {
			let line = Line::from (spans);
			lines.push (line);
		}
		Text::from (lines)
	}

}

impl <'tar> Target <'tar> for TextTargetBorrow <'tar> {

	fn width (& self) -> usize {
		self.width
	}

	fn push_str (& mut self, text: Cow <'tar, str>) {
		self.spans.push (Span::styled (text, self.attr));
	}

	fn push_attr (& mut self, attr: Attr) {
		self.attr = attr;
	}

	fn newline (& mut self) {
		let line = Line::from (mem::take (& mut self.spans));
		self.lines.push (line);
	}

}

pub struct TextTargetOwned {
	width: usize,
	lines: Vec <Line <'static>>,
	spans: Vec <Span <'static>>,
	attr: Attr,
}

impl TextTargetOwned {

	pub fn new (width: usize) -> Self {
		Self {
			width,
			lines: Vec::new (),
			spans: Vec::new (),
			attr: Attr::default (),
		}
	}

	pub fn build <'tar> (self) -> Text <'tar> {
		let Self { mut lines, spans, .. } = self;
		if ! spans.is_empty () {
			let line = Line::from (spans);
			lines.push (line);
		}
		Text::from (lines)
	}

}

impl <'tar> Target <'tar> for TextTargetOwned {

	fn width (& self) -> usize {
		self.width
	}

	fn push_str (& mut self, text: Cow <'tar, str>) {
		self.spans.push (Span::styled (text.into_owned (), self.attr));
	}

	fn push_attr (& mut self, attr: Attr) {
		self.attr = attr;
	}

	fn newline (& mut self) {
		let line = Line::from (mem::take (& mut self.spans));
		self.lines.push (line);
	}

}
