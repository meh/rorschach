use std::{ptr, mem};
use std::io::{self, Read};
use std::any::{Any, TypeId};
use ansi_term::Style;
use {Field, Endian, util};

#[derive(Clone, PartialEq, Debug)]
pub struct Named {
	bits: usize,

	name:   String,
	style:  Option<Style>,
	binary: bool,

	endian: Endian,
	kind:   Option<TypeId>,
}

impl Named {
	pub fn bits(&self) -> usize {
		self.bits
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn style(&self) -> Option<Style> {
		self.style
	}

	pub fn binary(&self) -> bool {
		self.binary
	}

	pub fn endian(&self) -> Endian {
		self.endian
	}

	pub fn kind(&self) -> Option<TypeId> {
		self.kind
	}

	pub fn read<R: Read>(&self, mut buffer: R) -> io::Result<Vec<u8>> {
		let mut data = vec![0u8; util::bytes(self.bits)];
		try!(buffer.read(&mut data));

		Ok(data)
	}

	pub fn decode<T: Any, R: Read>(&self, buffer: R) -> io::Result<T> {
		if self.kind.is_none() || TypeId::of::<T>() != self.kind.unwrap() {
			return Err(io::Error::new(io::ErrorKind::InvalidInput, "type mismatch"));
		}

		let mut data = try!(self.read(buffer));

		if cfg!(target_endian = "big") && self.endian == Endian::Little ||
		   cfg!(target_endian = "little") && self.endian == Endian::Big {
			data.reverse();
		}

		Ok(unsafe {
			ptr::read(data.as_ptr() as *const _ as *const T)
		})
	}
}

#[derive(Default)]
pub struct Builder {
	bits: Option<usize>,

	name:   Option<String>,
	style:  Option<Style>,
	binary: bool,

	endian: Option<Endian>,
	kind:   Option<TypeId>,
}

impl Builder {
	pub fn bits(mut self, value: usize) -> Self {
		self.bits = Some(value);
		self
	}

	pub fn bytes(self, value: usize) -> Self {
		self.bits(value * 8)
	}

	pub fn name<T: Into<String>>(mut self, name: T) -> Self {
		self.name = Some(name.into());
		self
	}

	pub fn style(mut self, value: Style) -> Self {
		self.style = Some(value);
		self
	}

	pub fn binary(mut self) -> Self {
		self.binary = true;
		self
	}

	pub fn is<T: Any>(mut self, endian: Endian) -> Self {
		self.bits   = Some(mem::size_of::<T>() * 8);
		self.endian = Some(endian);
		self.kind   = Some(TypeId::of::<T>());
		self
	}
}

impl Into<Field> for Builder {
	fn into(self) -> Field {
		Field::Named(Named {
			bits: self.bits.unwrap_or(0),

			name:   self.name.unwrap(),
			style:  self.style,
			binary: self.binary,

			endian: self.endian.unwrap_or(Default::default()),
			kind:   self.kind,
		})
	}
}
