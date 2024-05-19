use std::iter;

use crate::{ Attr, Colour, Printable, Target };

#[ derive (Clone, Copy) ]
pub struct BoxStyle {
	pub surround: Colour,
	pub background: Colour,
	pub foreground: Colour,
}

impl BoxStyle {

	pub const fn new (surround: Colour, background: Colour, foreground: Colour) -> Self {
		Self { surround, background, foreground }
	}

	pub fn with_width (self, width: usize) -> BorderBox {
		BorderBox::new (self, width)
	}

	pub fn border_attr (self) -> Attr {
		Attr::default ()
			.background (self.surround)
			.foreground (self.background)
	}

	pub fn text_attr (self) -> Attr {
		Attr::default ()
			.background (self.background)
			.foreground (self.foreground)
	}

}

#[ derive (Clone, Copy) ]
pub struct BorderBox {
	pub box_style: BoxStyle,
	pub width: usize,
}

impl BorderBox {

	pub fn new (box_style: BoxStyle, width: usize) -> Self {
		Self { box_style, width }
	}

	pub fn top (self) -> Border {
		Border::top (self.box_style.surround, self.box_style.background, self.width)
	}

	pub fn bottom (self) -> Border {
		Border::bottom (self.box_style.surround, self.box_style.background, self.width)
	}

	pub fn separator (self) -> Border {
		Border::middle (self.box_style.surround, self.box_style.background, self.width)
	}

	pub fn text_attr (self) -> Attr {
		self.box_style.text_attr ()
	}

}

#[ derive (Clone, Copy) ]
pub struct Border {
	type_: BorderType,
	background: Colour,
	foreground: Colour,
	width: usize,
}

#[ derive (Clone, Copy) ]
pub enum BorderType {
	Top,
	Bottom,
	Middle,
}

impl Border {

	const SYM_TOP: & 'static str = "▅";
	const SYM_BOTTOM: & 'static str = "🮄";
	const SYM_MIDDLE: & 'static str = "─";
	//const SYM_MIDDLE: & 'static str = "━";

	pub fn new (type_: BorderType, background: Colour, foreground: Colour, width: usize) -> Self {
		Self { type_, background, foreground, width }
	}

	pub fn top (background: Colour, foreground: Colour, width: usize) -> Self {
		Self::new (BorderType::Top, background, foreground, width)
	}

	pub fn bottom (background: Colour, foreground: Colour, width: usize) -> Self {
		Self::new (BorderType::Bottom, background, foreground, width)
	}

	pub fn middle (background: Colour, foreground: Colour, width: usize) -> Self {
		Self::new (BorderType::Middle, background, foreground, width)
	}

}

impl <'dat> Printable <'dat> for Border {

	fn print (self, target: & mut impl Target <'dat>) {
		let Self { type_, background, foreground, width } = self;
		let (background, foreground, ch) = match type_ {
			BorderType::Top => (background, foreground, Self::SYM_TOP),
			BorderType::Bottom => (background, foreground, Self::SYM_BOTTOM),
			BorderType::Middle => (foreground, background, Self::SYM_MIDDLE),
		};
		target.push (Attr::default ().background (background).foreground (foreground));
		target.push (iter::repeat (ch).take (width).collect::<String> ());
		target.push (Attr::default ());
	}

}
