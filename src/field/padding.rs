use std::io::{self, Read};
use {Field};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Padding {
	bits: usize,
}

impl Padding {
	pub fn bits(&self) -> usize {
		self.bits
	}

	pub fn read<R: Read>(&self, mut buffer: R) -> io::Result<Vec<u8>> {
		let mut data = vec![0u8; super::bytes(self.bits)];
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
		Field::Padding(Padding {
			bits: self.bits.expect("missing field size"),
		})
	}
}
