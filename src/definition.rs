use {Field};

/// A packet definition.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct Definition {
	fields: Vec<Field>,
}

impl Definition {
	/// The fields in the definition.
	pub fn fields(&self) -> &[Field] {
		self.fields.as_ref()
	}

	/// Add a field.
	pub fn field<T: Into<Field>>(mut self, field: T) -> Self {
		self.fields.push(field.into());
		self
	}
}
