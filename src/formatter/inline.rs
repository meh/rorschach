use std::io::{self, Read, Write};
use ansi_term;
use {Formatter, Definition, Field};
use super::Style;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Inline {
	split:   usize,
	newline: bool,
	style:   Option<Style>,
}

impl Default for Inline {
	fn default() -> Self {
		Inline {
			split:   1,
			newline: false,
			style:   None,
		}
	}
}

impl Inline {
	pub fn split(mut self, value: usize) -> Self {
		self.split = value;
		self
	}

	pub fn newline(mut self, value: bool) -> Self {
		self.newline = value;
		self
	}

	pub fn style(mut self, value: Style) -> Self {
		self.style = Some(value);
		self
	}
}

impl Formatter for Inline {
	fn format<R: Read, W: Write>(&self, def: &Definition, mut input: R, output: W) -> io::Result<()> {
		let mut printer = Printer { newline: self.newline, split: self.split, printed: 0, output: output };

		for field in def.fields() {
			match field {
				&Field::Named(ref field) => {
					try!(printer.print(&try!(field.read(input.by_ref())),
						self.style.and(field.style())));
				}

				&Field::Constant(ref field) => {
					let data = try!(field.read(input.by_ref()));

					try!(printer.print(&data,
						self.style.map(|s| if data == field.value() {
							s.constant.0
						}
						else {
							s.constant.1
						})));
				}

				&Field::Garbage(ref field) => {
					try!(printer.print(&try!(field.read(input.by_ref())),
						self.style.map(|s| s.garbage)));
				}

				&Field::Unknown(ref field) => {
					try!(printer.print(&try!(field.read(input.by_ref())),
						self.style.map(|s| s.unknown)));
				}

				&Field::Padding(ref field) => {
					for byte in try!(field.read(input.by_ref())) {
						try!(printer.print(&[byte][..],
							self.style.map(|s| if byte != 0 {
								s.padding.1
							}
							else {
								s.padding.0
							})));
					}
				}
			}
		}

		printer.finish()
	}
}

struct Printer<W: Write> {
	split:   usize,
	newline: bool,
	printed: usize,
	output:  W,
}

impl<W: Write> Printer<W> {
	pub fn print(&mut self, input: &[u8], style: Option<ansi_term::Style>) -> io::Result<()> {
		for byte in input {
			let string = format!("{:02x}", byte);

			if let Some(style) = style {
				try!(write!(self.output, "{}", style.paint(string)));
			}
			else {
				try!(write!(self.output, "{}", string));
			}

			if self.split > 0 && (self.printed + 1) % self.split == 0 {
				try!(write!(self.output, " "));
			}

			self.printed += 1;
		}

		Ok(())
	}

	pub fn finish(&mut self) -> io::Result<()> {
		if self.newline {
			try!(writeln!(self.output, ""));
		}

		Ok(())
	}
}
