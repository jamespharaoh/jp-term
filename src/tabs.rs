use std::borrow::Cow;
use std::iter;

use crate::{ Attr, Printable, Target };
use crate::border_box::BoxStyle;

pub struct BoxTabs <'dat> {
	tabs: Vec <(Cow <'dat, str>, BoxStyle)>,
}

impl <'dat> BoxTabs <'dat> {
	pub fn new (tabs: Vec <(Cow <'dat, str>, BoxStyle)>) -> Self {
		Self { tabs }
	}
}

impl <'tar> Printable <'tar> for BoxTabs <'tar> {
	fn print (self, target: & mut impl Target <'tar>) {

		let border_boxes: Vec <_> = self.tabs.iter ()
			.map (|& (ref label, box_style)| {
				box_style.with_width (label.chars ().count () + 4)
			})
			.collect ();

		for & border_box in & border_boxes {
			target.push (" ");
			target.push (border_box.top ());
		}
		target.newline ();

		for (& border_box, (label, _)) in iter::zip (& border_boxes, self.tabs) {
			target.push (" ");
			target.push (border_box.text_attr ());
			target.push ("  ");
			target.push (label);
			target.push ("  ");
			target.push (Attr::default ());
		}
		target.newline ();

		for & border_box in & border_boxes {
			target.push (" ");
			target.push (border_box.bottom ());
		}
		target.newline ();

	}
}

pub struct SlimTabs <'dat> {
	background: Attr,
	tabs: Vec <(Cow <'dat, str>, Attr)>,
}

impl <'dat> SlimTabs <'dat> {
	pub fn new (background: Attr, tabs: Vec <(Cow <'dat, str>, Attr)>) -> Self {
		Self { background, tabs }
	}
}

impl <'tar> Printable <'tar> for SlimTabs <'tar> {
	fn print (self, target: & mut impl Target <'tar>) {

		let mut pos = 0;

		for (label, attr) in self.tabs {
			pos += label.chars ().count () + 3;
			target.push (self.background);
			target.push (" ");
			target.push (attr);
			target.push (" ");
			target.push (label);
			target.push (" ");
		}

		target.push (self.background);
		while pos < target.width () {
			target.push (" ");
			pos += 1;
		}

		target.newline ();

	}
}
