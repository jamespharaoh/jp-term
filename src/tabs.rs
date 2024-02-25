use std::borrow::Cow;
use std::iter;

use crate::{ Attr, Printable, Target };
use crate::border_box::BoxStyle;

pub struct Tabs <'dat> {
	tabs: Vec <(Cow <'dat, str>, BoxStyle)>,
}

impl <'dat> Tabs <'dat> {
	pub fn new (tabs: Vec <(Cow <'dat, str>, BoxStyle)>) -> Self {
		Self { tabs }
	}
}

impl <'tar> Printable <'tar> for Tabs <'tar> {
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
