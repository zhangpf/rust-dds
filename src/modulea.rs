pub struct StructA {
	pub x: int,
}

#[allow(dead_code)]
pub enum EnumA {
    Cat = 1,
    Dog = 2,
}

impl StructA {
	pub	fn func_a(&self) -> (int, int) {
		(self.x, self.x)
	}
	pub fn func_b(&mut self, &mut new_x: int) {
		let tmp :int = self.x;
		self.x = *new_x;
		*new_x = tmp;
	}
}