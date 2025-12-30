use std::str::Chars;

pub struct Scanner<'a> {

	iterator: Chars<'a>,

	line: u32,

}

impl<'a> Scanner<'a> {

	pub fn new(source: &'a str) -> Self {
		Self { iterator: source.chars(), line: 1 }
	}

}
