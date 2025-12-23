pub type Value = f64;

pub fn print_value(value: Value) {
	print!("{}", value);
}

pub struct ValueArray {

	pub values: Vec<Value>,

}

impl ValueArray {

	pub fn new() -> Self {
		ValueArray { values: Vec::with_capacity(8) }
	}

	pub fn write(&mut self, value: f64) {
		self.values.push(value);
	}

}
