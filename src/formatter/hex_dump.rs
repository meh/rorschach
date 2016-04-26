use std::fmt;
use std::any::{TypeId};
use std::io::{self, Read, Write};
use ansi_term;
use {Formatter, Definition, Field};
use super::Style;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HexDump {
	width: usize,
	style: Option<Style>,
}

impl Default for HexDump {
	fn default() -> Self {
		HexDump {
			width: 0x10,
			style: None,
		}
	}
}

impl HexDump {
	pub fn width(mut self, value: usize) -> Self {
		self.width = value;
		self
	}

	pub fn style(mut self, value: Style) -> Self {
		self.style = Some(value);
		self
	}
}
