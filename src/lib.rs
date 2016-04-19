extern crate byteorder;
extern crate ansi_term;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Endian {
	Little,
	Big,
}

impl Default for Endian {
	#[cfg(target_endian = "big")]
	fn default() -> Self {
		Endian::Big
	}

	#[cfg(target_endian = "little")]
	fn default() -> Self {
		Endian::Little
	}
}

pub use Endian::{Little as LittleEndian, Big as BigEndian};

mod util;

mod definition;
pub use definition::Definition;

pub mod field;
pub use field::Field;

pub mod formatter;
pub use formatter::Formatter;
