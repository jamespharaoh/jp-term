use std::borrow::Cow;
use std::cmp;

use crate::{ Attr, Printable, Target };
use crate::border_box::BoxStyle;

pub struct MessageBox <'dat> {
	box_style: BoxStyle,
	text: Cow <'dat, str>,
}

impl <'dat> MessageBox <'dat> {

	#[ inline ]
	pub fn new (
		box_style: BoxStyle,
		text: impl Into <Cow <'dat, str>>,
	) -> Self {
		Self {
			box_style,
			text: text.into (),
		}
	}

}

impl <'dat> Printable <'dat> for MessageBox <'dat> {

	fn print (self, target: & mut impl Target <'dat>) {

		let box_style = self.box_style;
		let text = & * self.text;
		let (lines, text_width) = split (text, target.width ());
		let border_box = box_style.with_width (text_width + 4);

		target.push (" ");
		target.push (border_box.top ());
		target.newline ();

		for & (start, end) in & lines {
			let text = & text [start .. end];
			target.push (" ");
			target.push (box_style.text_attr ());
			let mut buf = String::new ();
			buf.push_str ("  ");
			buf.push_str (text);
			for _ in text.chars ().count () .. text_width {
				buf.push (' ');
			}
			buf.push_str ("  ");
			target.push (buf);
			target.push (Attr::default ());
			target.newline ();
		}

		target.push (" ");
		target.push (border_box.bottom ());
		target.newline ();

	}

}

pub struct MiniMessageBox <'dat> {
	box_style: BoxStyle,
	text: Cow <'dat, str>,
	width: usize,
}

impl <'dat> MiniMessageBox <'dat> {

	#[ inline ]
	pub fn new (
		box_style: BoxStyle,
		text: impl Into <Cow <'dat, str>>,
	) -> Self {
		let text = text.into ();
		let width = text.chars ().count () + 4;
		Self { box_style, text, width }
	}

	#[ inline ]
	pub fn width (& self) -> usize {
		self.width
	}

}

impl <'dat> Printable <'dat> for MiniMessageBox <'dat> {

	fn print (self, target: & mut impl Target <'dat>) {
		let box_style = self.box_style;
		target.push (box_style.border_attr ());
		target.push ("🬫");
		target.push (box_style.text_attr ());
		target.push (" ");
		target.push (self.text);
		target.push (" ");
		target.push (box_style.border_attr ());
		target.push ("🬛");
		target.push (Attr::default ());
	}

}

fn split (text: & str, max_width: usize) -> (Vec <(usize, usize)>, usize) {
	let mut lines = Vec::new ();
	let text_trim = text.trim_end ();
	let mut chars = text_trim.char_indices ();
	let mut line_pos = 0;
	let mut line_chars = 0;
	let mut prev_pos = 0;
	let mut text_width = 0;
	loop {
		match chars.next () {
			Some ((pos, '\n')) => {
				lines.push ((line_pos, pos));
				line_pos = pos + 1;
				text_width = cmp::max (text_width, line_chars);
				line_chars = 0;
				prev_pos = pos;
			},
			Some ((pos, _)) => {
				if line_chars == max_width {
					lines.push ((line_pos, prev_pos));
					line_pos = prev_pos;
					text_width = cmp::max (text_width, line_chars);
					line_chars = 0;
				}
				line_chars += 1;
				prev_pos = pos;
			},
			None => {
				if line_pos < text_trim.chars ().count () {
					lines.push ((line_pos, text_trim.chars ().count ()));
					text_width = cmp::max (text_width, line_chars);
				}
				break;
			},
		}
	}
	(lines, text_width)
}
