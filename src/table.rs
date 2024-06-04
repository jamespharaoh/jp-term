use std::cmp;
use std::fmt::{ self, Display };
use std::rc::Rc;

use crate::{ Attr, Printable, Target };
use crate::border_box::BoxStyle;

#[ derive (Clone, Copy, Debug) ]
pub enum Align {
	Left,
	Centre,
	Right,
}

#[ derive (Clone, Debug) ]
pub struct Table {
	data: Vec <Row>,
	widths: Rc <[u16]>,
	width: u16,
}

impl Table {

	pub fn builder () -> TableBuilder {
		TableBuilder {
			data: Vec::new (),
		}
	}

	pub fn rows (& self) -> impl Iterator <Item = TableRow> + '_ {
		self.data.iter ()
			.map (|row| match row {
				& Row::Cells { ref data } => TableRow::Cells {
					data: Rc::clone (data),
					widths: Rc::clone (& self.widths),
				},
				& Row::Separator => TableRow::Separator,
			})
	}

	#[ inline ]
	pub fn width (& self) -> u16 {
		self.width
	}

}

#[ derive (Clone, Debug) ]
pub enum TableRow {
	Cells {
		data: Rc <[Cell]>,
		widths: Rc <[u16]>,
	},
	Separator,
}

impl TableRow {

	#[ allow (dead_code) ]
	#[ inline ]
	pub fn is_cells (& self) -> bool {
		matches! (self, TableRow::Cells { .. })
	}

	#[ inline ]
	pub fn is_separator (& self) -> bool {
		matches! (self, TableRow::Separator)
	}

}

impl Display for TableRow {

	fn fmt (& self, fmtr: & mut fmt::Formatter) -> fmt::Result {
		match self {
			& TableRow::Cells { ref data, ref widths } => {
				let mut start = 0;
				for cell in & ** data {
					let end = start + cell.span as usize;
					let width = widths [start .. end].iter ().sum::<u16> () as usize;
					match cell.align {
						Align::Left => write! (fmtr, "{:<width$}", cell.text) ?,
						Align::Centre => write! (fmtr, "{:^width$}", cell.text) ?,
						Align::Right => write! (fmtr, "{:>width$}", cell.text) ?,
					}
					start = end;
				}
			},
			& TableRow::Separator => {
				write! (fmtr, "---") ?;
			},
		}
		Ok (())
	}

}

#[ derive (Clone, Debug) ]
pub struct TableBuilder {
	data: Vec <Row>,
}

impl TableBuilder {

	#[ inline ]
	pub fn row <'tab> (& 'tab mut self) -> RowBuilder <'tab> {
		RowBuilder {
			table_builder: self,
			data: Vec::new (),
		}
	}

	#[ inline ]
	pub fn separator (& mut self) {
		self.data.push (Row::Separator);
	}

	#[ inline ]
	pub fn build (self) -> Table {
		let widths = self.calc_widths ();
		let width = widths.iter ().sum ();
		Table {
			data: self.data,
			widths: Rc::from (& * widths),
			width,
		}
	}

	fn calc_widths (& self) -> Vec <u16> {
		let num_cols: u16 =
			self.data.iter ()
				.map (|row| match row {
					Row::Cells { data } => data.iter ().map (|cell| cell.span).sum (),
					Row::Separator => 0,
				})
				.max ()
				.unwrap_or_default ();
		let mut widths = vec! [0; num_cols as usize];
		for span in 1 ..= num_cols {
			for row in & self.data {
				let mut start = 0;
				let & Row::Cells { data: ref row_data } = row else { continue };
				for cell in & ** row_data {
					let end = start + cell.span;
					if cell.span == span {
						let mut cur_width: u16 = widths [start as usize .. end as usize].iter ().sum ();
						while cur_width < cell.width {
							let col_idx = (start .. end)
								.map (|idx| (idx, widths [idx as usize]))
								.min_by_key (|& (_, width)| width)
								.map (|(idx, _)| idx)
								.unwrap ();
							widths [col_idx as usize] += 1;
							cur_width += 1;
						}
					}
					start = end;
				}
			}
		}
		widths
	}

}

#[ derive (Clone, Debug) ]
pub enum Row {
	Cells { data: Rc <[Cell]> },
	Separator,
}

pub struct RowBuilder <'tab> {
	table_builder: & 'tab mut TableBuilder,
	data: Vec <Cell>,
}

impl <'tab> RowBuilder <'tab> {

	#[ inline ]
	pub fn cell (
		& mut self,
		span: u16,
		align: Align,
		width: u16,
		text: impl Into <String>,
	) -> & mut Self {
		self.cell_real (span, align, width, text.into ())
	}

	fn cell_real (
		& mut self,
		span: u16,
		align: Align,
		width: u16,
		text: String,
	) -> & mut Self {
		let width = cmp::max (width, text.chars ().count ().try_into ().unwrap ());
		self.data.push (Cell { span, align, width, text });
		self
	}

	#[ inline ]
	pub fn left (& mut self, text: impl Into <String>) -> & mut Self {
		self.cell (1, Align::Left, 0, text)
	}

	#[ inline ]
	pub fn left_span (& mut self, span: u16, text: impl Into <String>) -> & mut Self {
		self.cell (span, Align::Left, 0, text)
	}

	#[ inline ]
	pub fn centre (& mut self, text: impl Into <String>) -> & mut Self {
		self.cell (1, Align::Centre, 0, text)
	}

	#[ inline ]
	pub fn centre_span (& mut self, span: u16, text: impl Into <String>) -> & mut Self {
		self.cell (span, Align::Centre, 0, text)
	}

	#[ inline ]
	pub fn right (& mut self, text: impl Into <String>) -> & mut Self {
		self.cell (1, Align::Right, 0, text)
	}

	#[ inline ]
	pub fn right_span (& mut self, span: u16, text: String) -> & mut Self {
		self.cell (span, Align::Right, 0, text)
	}

	#[ inline ]
	pub fn space (& mut self, width: u16) -> & mut Self {
		self.cell (1, Align::Left, width, String::new ())
	}

	#[ inline ]
	pub fn space_span (& mut self, span: u16, width: u16) -> & mut Self {
		self.cell (span, Align::Left, width, String::new ())
	}

	#[ inline ]
	pub fn empty (& mut self) -> & mut Self {
		self.cell (1, Align::Left, 0, String::new ())
	}

	#[ inline ]
	pub fn empty_span (& mut self, span: u16) -> & mut Self {
		self.cell (span, Align::Left, 0, String::new ())
	}

	pub fn build (self) {
		self.table_builder.data.push (Row::Cells {
			data: Rc::from (self.data),
		});
	}

}

#[ derive (Clone, Debug) ]
pub struct Cell {
	span: u16,
	align: Align,
	width: u16,
	text: String,
}

pub struct TableBox <'tab> {
	box_style: BoxStyle,
	table: & 'tab Table,
}

impl <'tab> TableBox <'tab> {

	#[ inline ]
	pub fn new (box_style: BoxStyle, table: & 'tab Table) -> Self {
		Self { box_style, table }
	}

}

impl <'tab, 'dat> Printable <'dat> for TableBox <'tab> {

	fn print (self, target: & mut impl Target <'dat>) {

		let full_width = self.table.width () as usize + 4;
		let border_box = self.box_style.with_width (full_width);

		target.push (" ");
		target.push (border_box.top ());
		target.newline ();

		for row in self.table.rows () {
			if row.is_separator () {
				target.push (" ");
				target.push (border_box.separator ());
				target.newline ();
			} else {
				target.push (" ");
				target.push (self.box_style.text_attr ());
				target.push ("  ");
				target.push (row.to_string ());
				target.push ("  ");
				target.push (Attr::default ());
				target.newline ();
			}
		}

		target.push (" ");
		target.push (border_box.bottom ());
		target.newline ();

	}

}
