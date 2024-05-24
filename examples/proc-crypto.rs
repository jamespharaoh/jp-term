use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::mem;

use jp_term::Colour;
use jp_term::Target;
use jp_term::ansi::AnsiTarget;
use jp_term::border_box::BoxStyle;
use jp_term::table::Table;
use jp_term::table::TableBox;

fn main () -> anyhow::Result <()> {
	let cryptos = read () ?;
	let mut target = AnsiTarget::new (io::stdout (), 80);
	render (& mut target, & cryptos);
	Ok (())
}

fn read () -> anyhow::Result <Vec <Crypto>> {
	let mut cryptos = Vec::new ();
	let mut crypto = Crypto::default ();
	for line in BufReader::new (File::open ("/proc/crypto") ?).lines () {
		let line = line ?;
		let line = line.trim ();
		if line.is_empty () {
			cryptos.push (mem::take (& mut crypto));
		} else {
			let mut parts = line.split (" : ");
			let name = parts.next ().unwrap_or_default ().trim ();
			let value = parts.next ().unwrap_or_default ().trim ();
			let target = match name {
				"name" => Some (& mut crypto.name),
				"driver" => Some (& mut crypto.driver),
				"module" => Some (& mut crypto.module),
				"type" => Some (& mut crypto.type_),
				_ => None,
			};
			if let Some (target) = target {
				target.clear ();
				target.push_str (value);
			}
		}
	}
	Ok (cryptos)
}

fn render <'tar> (target: & mut impl Target <'tar>, cryptos: & [Crypto]) {
	let mut table = Table::builder ();
	let mut row = table.row ();
	row
		.left ("Name")
		.space (2)
		.left ("Driver")
		.space (2)
		.left ("Module")
		.space (2)
		.left ("Type");
	row.build ();
	table.separator ();
	for crypto in cryptos {
		let mut row = table.row ();
		row
			.left (& crypto.name)
			.space (2)
			.left (& crypto.driver)
			.space (2)
			.left (& crypto.module)
			.space (2)
			.left (& crypto.type_);
		row.build ();
	}
	let table = table.build ();
	let box_style = BoxStyle::new (
		Colour::new (0x00, 0x00, 0x00),
		Colour::new (0x50, 0x40, 0x30),
		Colour::new (0xff, 0xff, 0xff));
	let table_box = TableBox::new (box_style, & table);
	target.push (table_box);
}

#[ derive (Default) ]
struct Crypto {
	name: String,
	driver: String,
	module: String,
	type_: String,
}
