use std::io::{self, Read, Write};
use ansi_term;
use {Definition};

pub trait Formatter {
	fn format<R: Read, W: Write>(&self, def: &Definition, input: R, output: W) -> io::Result<()>;
}

pub use ansi_term::Colour as Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Style {
	pub default:  ansi_term::Style,
	pub constant: (ansi_term::Style, ansi_term::Style),
	pub padding:  (ansi_term::Style, ansi_term::Style),
	pub garbage:  ansi_term::Style,
	pub unknown:  ansi_term::Style,
}

impl Default for Style {
	fn default() -> Self {
		Style {
			default:  Color::White.normal(),
			constant: (Color::Fixed(237).normal(), Color::Fixed(255).on(Color::Red)),
			padding:  (Color::Fixed(237).normal(), Color::Fixed(255).on(Color::Red)),
			garbage:  Color::Fixed(237).normal(),
			unknown:  Color::Fixed(240).normal(),
		}
	}
}

mod structured;
pub use self::structured::Structured;

mod inline;
pub use self::inline::Inline;
