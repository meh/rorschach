use std::slice;
use std::io::{self, Read};
use {Field, util};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Constant {
	bits: usize,

	value: Vec<u8>,
}

impl Constant {
	pub fn bits(&self) -> usize {
		self.bits
	}

	pub fn value(&self) -> &[u8] {
		&self.value
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

	value: Option<Vec<u8>>,
}

impl Builder {
	pub fn bits(mut self, value: usize) -> Self {
		self.bits = Some(value);
		self
	}

	pub fn bytes(self, value: usize) -> Self {
		self.bits(value * 8)
	}

	pub fn value<T: 'static>(mut self, value: T) -> Self {
		self.value = Some(unsafe {
			slice::from_raw_parts(&value as *const _ as *const u8,
			                      util::bytes(self.bits.unwrap_or(0)))
		}.to_vec());

		self
	}
}

impl Into<Field> for Builder {
	fn into(self) -> Field {
		Field::Constant(Constant {
			bits:  self.bits.unwrap_or(0),
			value: self.value.unwrap_or(Vec::new()),
		})
	}
}
