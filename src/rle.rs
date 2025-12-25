pub struct RunLengthEncoder<T: PartialEq> {
	values: Vec<(T, u32)>,
}

impl<T: PartialEq> RunLengthEncoder<T> {

	pub fn new() -> Self {
		Self { values: Vec::with_capacity(8) }
	}

	pub fn add(&mut self, value: T) {
		match self.values.last_mut() {
			Some(v) if v.0 == value => v.1 += 1,
			_ => self.values.push((value, 1))
		};
	}

	pub fn find(&self, position: usize) -> Option<&T> {
		let mut lower_bound: usize = 0;
		for tuple in &self.values {
			let upper_bound = lower_bound + tuple.1 as usize;
			if position >= lower_bound && position < upper_bound {
				return Some(&tuple.0);
			} else {
				lower_bound += tuple.1 as usize;
			}
		}
		None
	}

}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add_should_not_increase_valueslen_when_same_value_added_twice() {
		let mut sut = RunLengthEncoder::<u32>::new();

		sut.add(1);
		sut.add(1);

		assert_eq!(sut.values.len(), 1);
		assert_eq!(sut.values.last().unwrap().0, 1);
		assert_eq!(sut.values.last().unwrap().1, 2);
	}

	#[test]
	fn add_should_increase_valueslen_when_different_values_added() {
		let mut sut = RunLengthEncoder::<u32>::new();

		sut.add(1);
		sut.add(2);

		assert_eq!(sut.values.len(), 2);
		assert_eq!(sut.values[0].1, 1);
		assert_eq!(sut.values[1].1, 1);
	}

	#[test]
	fn find_should_return_some_when_position_in_range() {
		let mut sut = RunLengthEncoder::<u32>::new();
		sut.add(100);
		sut.add(100);
		sut.add(100);
		sut.add(200);

		let end_of_first_range = *sut.find(2).unwrap();
		let start_of_second_range = *sut.find(3).unwrap();

		assert_eq!(end_of_first_range, 100);
		assert_eq!(start_of_second_range, 200);
	}

	#[test]
	fn find_should_return_none_when_position_out_of_range() {
		let mut sut = RunLengthEncoder::<u32>::new();
		sut.add(100);
		sut.add(100);
		sut.add(200);

		let result = sut.find(3);

		assert!(result.is_none());
	}

}
