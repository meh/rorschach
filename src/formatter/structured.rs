use std::fmt;
use std::any::{TypeId};
use std::io::{self, Read, Write};
use ansi_term;
use {Formatter, Definition, Field};
use super::Style;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Structured {
	header: bool,
	style:  Option<Style>,
}

impl Structured {
	pub fn header(mut self, value: bool) -> Self {
		self.header = value;
		self
	}

	pub fn style(mut self, value: Style) -> Self {
		self.style = Some(value);
		self
	}
}

impl Formatter for Structured {
	fn format<R: Read, W: Write>(&self, def: &Definition, mut input: R, mut output: W) -> io::Result<()> {
		if self.header {
			try!(writeln!(output, " 0                   1                   2                   3"));
			try!(writeln!(output, " 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1"));
			try!(writeln!(output, "+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+"));
		}

		let mut printer = Printer { consumed: 0, padding: 0, output: output.by_ref() };

		for field in def.fields() {
			match field {
				&Field::Named(ref field) => {
					let kind  = field.kind().unwrap_or(TypeId::of::<()>());
					let style = self.style.and(field.style());

					if kind == TypeId::of::<u8>() {
						try!(printer.print(field.decode::<u8, _>(input.by_ref()).unwrap(),
							field.bits(), style));
					}
					else if kind == TypeId::of::<u16>() {
						try!(printer.print(field.decode::<u16, _>(input.by_ref()).unwrap(),
							field.bits(), style));
					}
					else if kind == TypeId::of::<u32>() {
						try!(printer.print(field.decode::<u32, _>(input.by_ref()).unwrap(),
							field.bits(), style));
					}
					else if kind == TypeId::of::<i8>() {
						try!(printer.print(field.decode::<i8, _>(input.by_ref()).unwrap(),
							field.bits(), style));
					}
					else if kind == TypeId::of::<i16>() {
						try!(printer.print(field.decode::<i16, _>(input.by_ref()).unwrap(),
							field.bits(), style));
					}
					else if kind == TypeId::of::<i32>() {
						try!(printer.print(field.decode::<i32, _>(input.by_ref()).unwrap(),
							field.bits(), style));
					}
					else if field.binary() {
						try!(printer.binary(&try!(field.read(input.by_ref())),
							field.bits(), style));
					}
					else {
						try!(printer.hex(&try!(field.read(input.by_ref())),
							field.bits(), style));
					}
				}

				&Field::Constant(ref field) => {
					let data = try!(field.read(input.by_ref()));

					if data == field.value() {
						try!(printer.hex(&data,
							field.bits(), self.style.map(|s| s.constant.0)));
					}
					else {
						try!(printer.hex(&data,
							field.bits(), self.style.map(|s| s.constant.1)));
					}
				}

				&Field::Garbage(ref field) => {
					try!(printer.hex(&try!(field.read(input.by_ref())),
						field.bits(), self.style.map(|s| s.garbage)));
				}

				&Field::Unknown(ref field) => {
					try!(printer.hex(&try!(field.read(input.by_ref())),
						field.bits(), self.style.map(|s| s.unknown)));
				}

				&Field::Padding(ref field) => {
					for byte in try!(field.read(input.by_ref())) {
						if byte != 0 {
							try!(printer.byte(byte, self.style.map(|s| s.padding.1)));
						}
						else {
							try!(printer.pad());
						}
					}
				}
			}
		}

		printer.finish()
	}
}

struct Printer<W: Write> {
	consumed: usize,
	padding:  usize,
	output:   W,
}

impl<W: Write> Printer<W> {
	fn space(string: &str, width: usize) -> (usize, usize) {
		let rem  = width - string.len();
		let half = if rem % 2 == 0 { rem / 2 } else { (rem - 1) / 2 };

		if rem % 2 == 0 {
			(half, half)
		}
		else {
			(half + 1, half)
		}
	}

	pub fn pad(&mut self) -> io::Result<()> {
		self.padding += 1;

		if self.padding > 1 && self.consumed % 32 != 0 {
			try!(write!(self.output, " "));
		}
		else {
			try!(write!(self.output, "|"));
		}

		try!(write!(self.output, "{: ^15}", ""));

		self.done(8)
	}

	pub fn byte(&mut self, data: u8, style: Option<ansi_term::Style>) -> io::Result<()> {
		self.hex(&[data][..], 8, style)
	}

	pub fn binary(&mut self, data: &[u8], bits: usize, style: Option<ansi_term::Style>) -> io::Result<()> {
		self.print({
			let mut out = String::with_capacity(data.len() * 8);

			for byte in data {
				out.push_str(&format!("{:08b}", byte));
			}

			out.pop();
			out
		}, bits, style)
	}

	pub fn hex(&mut self, data: &[u8], bits: usize, style: Option<ansi_term::Style>) -> io::Result<()> {
		self.print({
			let mut out = String::with_capacity(data.len() * 3);

			for byte in data {
				out.push_str(&format!("{:02x} ", byte));
			}

			out.pop();
			out
		}, bits, style)
	}

	pub fn print<T: fmt::Display>(&mut self, data: T, bits: usize, style: Option<ansi_term::Style>) -> io::Result<()> {
		let string        = data.to_string();
		let width         = bits * 2 - 1;
		let (left, right) = Self::space(&string, width);

		try!(write!(self.output, "|"));
		try!(write!(self.output, "{: ^1$}", "", left));

		if let Some(style) = style {
			try!(write!(self.output, "{}", style.paint(string)));
		}
		else {
			try!(write!(self.output, "{}", string));
		}

		try!(write!(self.output, "{: ^1$}", "", right));

		self.padding = 0;
		self.done(bits)
	}

	pub fn finish(&mut self) -> io::Result<()> {
		if self.consumed % 32 == 0 {
			return Ok(());
		}

		try!(writeln!(self.output, "|"));

		for _ in 0 .. (self.consumed % 32) {
			try!(write!(self.output, "+-"));
		}

		try!(writeln!(self.output, "+"));

		Ok(())
	}

	fn done(&mut self, bits: usize) -> io::Result<()> {
		self.consumed += bits;

		if self.consumed % 32 == 0 {
			try!(writeln!(self.output, "|"));
			try!(writeln!(self.output, "+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+"));
		}

		Ok(())
	}
}
