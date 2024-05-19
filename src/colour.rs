use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

#[ derive (Clone, Copy, Debug, Default, Eq, PartialEq) ]
pub struct Colour {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

impl Colour {

	pub const fn new (red: u8, green: u8, blue: u8) -> Self {
		Self { red, green, blue }
	}

}

impl FromStr for Colour {

	type Err = ParseColourError;

	fn from_str (source: & str) -> Result <Self, ParseColourError> {
		let source_chars = source.chars ().count ();
		if source_chars == 4 {
			let first_char = source.chars ().next ().unwrap ();
			if first_char != '#' { return Err (ParseColourError::InvalidFirstChar (first_char)) }
			let value = u16::from_str_radix (& source [1 .. ], 16).map_err (|_| ParseColourError::ValueParse) ?;
			return Ok (Self {
				red: ((value & 0xf00) >> 8) as u8 * 0x11,
				green: ((value & 0x0f0) >> 4) as u8 * 0x11,
				blue: (value & 0x00f) as u8 * 0x11,
			});
		}
		if source_chars == 7 {
			let first_char = source.chars ().next ().unwrap ();
			if first_char != '#' { return Err (ParseColourError::InvalidFirstChar (first_char)) }
			let value = u32::from_str_radix (& source [1 .. ], 16).map_err (|_| ParseColourError::ValueParse) ?;
			return Ok (Self {
				red: ((value & 0xff0000) >> 16) as u8,
				green: ((value & 0x00ff00) >> 8) as u8,
				blue: (value & 0x0000ff) as u8,
			});
		}
		Err (ParseColourError::InvalidLength (source_chars))
	}

}

#[ derive (Debug, Eq, PartialEq, thiserror::Error) ]
pub enum ParseColourError {
	#[ error ("invalid length {0} (should be 4 or 7)") ]
	InvalidLength (usize),
	#[ error ("should start with '#', not '{0}'") ]
	InvalidFirstChar (char),
	#[ error ("invalid hex value") ]
	ValueParse,
}

pub struct ColoursMap {
	data: HashMap <String, Colour>,
}

impl ColoursMap {

	pub fn build (
		source: impl IntoIterator <Item = (impl AsRef <str>, impl AsRef <str>)>,
	) -> Result <Self, ColoursMapBuildError> {
		let mut remain: HashMap <String, String> = HashMap::new ();
		let mut data: HashMap <String, Colour> = HashMap::new ();
		for (name, value) in source {
			let name = name.as_ref ();
			let value = value.as_ref ();
			if value.chars ().next () == Some ('#') {
				data.insert (
					name.to_owned (),
					value.parse ()
						.map_err (|err| ColoursMapBuildError::ParseError (name.to_owned (), err)) ?);
			} else {
				remain.insert (name.to_owned (), value.to_owned ());
			}
		}
		for (name, value) in & remain {
			if ! remain.contains_key (value) && ! data.contains_key (value) {
				return Err (ColoursMapBuildError::InvalidReference (name.to_owned (), value.to_owned ()));
			}
		}
		while ! remain.is_empty () {
			let mut progress = false;
			remain.retain (|name, value| {
				let Some (& colour) = data.get (value) else { return true };
				data.insert (name.to_owned (), colour);
				progress = true;
				false
			});
			if ! progress {
				let (name, value) = remain.iter ().next ().unwrap ();
				return Err (ColoursMapBuildError::CircularReference (
					name.clone (),
					value.clone ()));
			}
		}
		Ok (Self { data })
	}

}

impl Deref for ColoursMap {

	type Target = HashMap <String, Colour>;

	fn deref (& self) -> & HashMap <String, Colour> {
		& self.data
	}

}

#[ derive (Debug, Eq, PartialEq, thiserror::Error) ]
pub enum ColoursMapBuildError {
	#[ error ("Parse error for {0}") ]
	ParseError (String, #[ source ] ParseColourError),
	#[ error ("Invalid colour name for {0}: {1}") ]
	InvalidReference (String, String),
	#[ error ("Circular reference for {0}: {1}") ]
	CircularReference (String, String),
}

#[ cfg (test) ]
mod tests {

	use super::*;

	#[ test ]
	fn colour_from_str () {
		assert_eq! (Ok (Colour::new (0x42, 0x9d, 0x4a)), Colour::from_str ("#429d4a"));
		assert_eq! (Ok (Colour::new (0xed, 0x4a, 0x56)), Colour::from_str ("#ED4A56"));
		assert_eq! (Ok (Colour::new (0xdd, 0xaa, 0x99)), Colour::from_str ("#da9"));
		assert_eq! (Ok (Colour::new (0xcc, 0x44, 0x22)), Colour::from_str ("#C42"));
		assert_eq! (Err (ParseColourError::InvalidLength (0)), Colour::from_str (""));
		assert_eq! (Err (ParseColourError::InvalidLength (5)), Colour::from_str ("#abcd"));
		assert_eq! (Err (ParseColourError::InvalidFirstChar ('w')), Colour::from_str ("wxyz"));
		assert_eq! (Err (ParseColourError::ValueParse), Colour::from_str ("#01g"));
	}

	#[ test ]
	fn colours_map_build_ok () -> anyhow::Result <(), ColoursMapBuildError> {
		let map = ColoursMap::build ([
			("black", "#000000"),
			("white", "#ffffff"),
			("default-foreground", "white"),
			("default-background", "black"),
			("inverse-foreground", "default-background"),
			("inverse-background", "default-foreground"),
		]) ?;
		assert_eq! (6, map.len ());
		assert_eq! (Some (& Colour::new (0x00, 0x00, 0x00)), map.get ("black"));
		assert_eq! (Some (& Colour::new (0xff, 0xff, 0xff)), map.get ("white"));
		assert_eq! (Some (& Colour::new (0xff, 0xff, 0xff)), map.get ("default-foreground"));
		assert_eq! (Some (& Colour::new (0x00, 0x00, 0x00)), map.get ("default-background"));
		assert_eq! (Some (& Colour::new (0x00, 0x00, 0x00)), map.get ("inverse-foreground"));
		assert_eq! (Some (& Colour::new (0xff, 0xff, 0xff)), map.get ("inverse-background"));
		Ok (())
	}

	#[ test ]
	fn colours_map_build_error () {
		assert_eq! (
			Some (ColoursMapBuildError::ParseError (
				"a".to_owned (),
				Colour::from_str ("#hello!").unwrap_err ())),
			ColoursMap::build ([ ("a", "#hello!") ]).err ());
		assert_eq! (
			Some (ColoursMapBuildError::InvalidReference ("a".to_owned (), "b".to_owned ())),
			ColoursMap::build ([ ("a", "b") ]).err ());
		assert_eq! (
			Some (ColoursMapBuildError::CircularReference ("a".to_owned (), "a".to_owned ())),
			ColoursMap::build ([ ("a", "a") ]).err ());
	}

}
