use {Field};

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Definition {
	fields: Vec<Field>,
}

impl Definition {
	pub fn fields(&self) -> &[Field] {
		self.fields.as_ref()
	}

	pub fn field<T: Into<Field>>(mut self, field: T) -> Self {
		self.fields.push(field.into());
		self
	}
}
