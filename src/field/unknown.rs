use std::io::{self, Read};
use {Field};

/// An unkown field, it means you're not sure whether the data is padding or
/// garbage.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Unknown {
	bits: usize,
}

impl Unknown {
	/// Get how many bits the field is.
	pub fn bits(&self) -> usize {
		self.bits
	}

	/// Read the data from a `Read` for this field.
	pub fn read<R: Read>(&self, mut buffer: R) -> io::Result<Vec<u8>> {
		let mut data = vec![0u8; super::bytes(self.bits)];
		try!(buffer.read(&mut data));

		Ok(data)
	}
}

/// A builder for an unknown field.
#[derive(Default)]
pub struct Builder {
	bits: Option<usize>,
}

impl Builder {
	/// Defines the size in bits.
	pub fn bits(mut self, value: usize) -> Self {
		self.bits = Some(value);
		self
	}

	/// Defines the size in bytes.
	pub fn bytes(self, value: usize) -> Self {
		self.bits(value * 8)
	}
}

impl Into<Field> for Builder {
	fn into(self) -> Field {
		Field::Unknown(Unknown {
			bits: self.bits.expect("missing field size"),
		})
	}
}
