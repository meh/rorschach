pub mod constant;
pub use self::constant::Constant;

pub mod garbage;
pub use self::garbage::Garbage;

pub mod unknown;
pub use self::unknown::Unknown;

pub mod padding;
pub use self::padding::Padding;

pub mod named;
pub use self::named::Named;

#[derive(Clone, PartialEq, Debug)]
pub enum Field {
	Constant(Constant),
	Garbage(Garbage),
	Unknown(Unknown),
	Padding(Padding),
	Named(Named),
}

impl Field {
	pub fn bits(&self) -> usize {
		match self {
			&Field::Constant(ref v) => v.bits(),
			&Field::Garbage(ref v)  => v.bits(),
			&Field::Unknown(ref v)  => v.bits(),
			&Field::Padding(ref v)  => v.bits(),
			&Field::Named(ref v)    => v.bits(),
		}
	}

	pub fn constant() -> constant::Builder {
		constant::Builder::default()
	}

	pub fn named<T: Into<String>>(name: T) -> named::Builder {
		named::Builder::default().name(name)
	}

	pub fn padding() -> padding::Builder {
		padding::Builder::default()
	}

	pub fn garbage() -> garbage::Builder {
		garbage::Builder::default()
	}

	pub fn unknown() -> unknown::Builder {
		unknown::Builder::default()
	}
}
