use std::io::{self, Read};
use {Field, util};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Unknown {
	bits: usize,
}

impl Unknown {
	pub fn bits(&self) -> usize {
		self.bits
	}

	pub fn read<R: Read>(&self, mut buffer: R) -> io::Result<Vec<u8>> {
		let mut data = vec![0u8; util::bytes(self.bits)];
		try!(buffer.read(&mut data));

		Ok(data)
	}
}

#[derive(Default)]
pub struct Builder {
	bits: Option<usize>,
}

impl Builder {
	pub fn bits(mut self, value: usize) -> Self {
		self.bits = Some(value);
		self
	}

	pub fn bytes(self, value: usize) -> Self {
		self.bits(value * 8)
	}
}

impl Into<Field> for Builder {
	fn into(self) -> Field {
		Field::Unknown(Unknown {
			bits: self.bits.unwrap_or(0),
		})
	}
}
