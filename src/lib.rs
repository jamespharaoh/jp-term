use std::borrow::Cow;

pub mod ansi;
pub mod colour;
pub mod border_box;
pub mod message_box;
pub mod ratatui;
pub mod table;
pub mod tabs;

pub use colour::Colour;

pub trait Target <'dat>: Sized {

	fn width (& self) -> usize;

	fn push_attr (& mut self, attr: Attr);
	fn push_str (& mut self, text: Cow <'dat, str>);
	fn newline (& mut self);

	#[ inline ]
	fn push (& mut self, value: impl Printable <'dat>) {
		value.print (self);
	}

}

pub trait Printable <'dat> {
	fn print (self, target: & mut impl Target <'dat>);
}

impl <'dat> Printable <'dat> for & 'dat str {
	#[ inline ]
	fn print (self, target: & mut impl Target <'dat>) {
		target.push_str (Cow::Borrowed (self));
	}
}

impl <'dat> Printable <'dat> for String {
	#[ inline ]
	fn print (self, target: & mut impl Target <'dat>) {
		target.push_str (Cow::Owned (self));
	}
}

impl <'dat> Printable <'dat> for Cow <'dat, str> {
	#[ inline ]
	fn print (self, target: & mut impl Target <'dat>) {
		target.push_str (self);
	}
}

#[ derive (Clone, Copy, Default) ]
pub struct Attr {
	foreground: Option <Colour>,
	background: Option <Colour>,
	bold: bool,
}

impl Attr {

	#[ inline ]
	pub const fn default () -> Self {
		Self {
			foreground: None,
			background: None,
			bold: false,
		}
	}

	#[ inline ]
	pub const fn foreground (mut self, foreground: Colour) -> Self {
		self.foreground = Some (foreground);
		self
	}

	#[ inline ]
	pub const fn background (mut self, background: Colour) -> Self {
		self.background = Some (background);
		self
	}

	#[ inline ]
	pub const fn bold (mut self, bold: bool) -> Self {
		self.bold = bold;
		self
	}

}

impl <'dat> Printable <'dat> for Attr {
	#[ inline ]
	fn print (self, target: & mut impl Target <'dat>) {
		target.push_attr (self);
	}
}

pub struct Term {
	pub size_rows: usize,
	pub size_cols: usize,
}

impl Term {

	pub fn new () -> Self {
		let size = termsize::get ();
		Self {
			size_rows: size.as_ref ().map (|size| size.rows).unwrap_or (25) as usize,
			size_cols: size.as_ref ().map (|size| size.cols).unwrap_or (80) as usize,
		}
	}

	pub fn reset (& mut self) {
		print! ("\x1bc");
	}

}

#[ macro_export ]
macro_rules! printable {
	(
		$(
			$name:ident <
				$tar:lifetime
				$( , $life:lifetime )*
				$( , )?
			> (
				$target:ident
				$( , $arg_name:ident : $arg_type:ty )*
				$( , )?
			) {
				$( $body:tt )*
			}
		)*
	) => {
		$(

			pub struct $name <$($life),*> {
				$( $arg_name: $arg_type, )*
			}

			impl <$($life),*> $name <$($life),*> {
				pub fn new ($( $arg_name: $arg_type ),*) -> Self {
					Self { $($arg_name),* }
				}
			}

			impl <'tar, $($life),*> term::Printable <'tar> for $name <$($life),*> {
				fn print (self, $target: & mut impl term::Target <'tar>) {
					let Self { $($arg_name),* } = self;
					$($body)*
				}
			}

		)*
	}
}
