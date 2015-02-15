use std::default::Default;
use err::DDSError;

pub struct CombiantionSet<
	T1: PartialEq + Copy, 
	T2: PartialEq + Copy> {
	
	set: Vec<(T1, T2)>,
}

impl<T1: PartialEq + Copy, 
	 T2: PartialEq + Copy> Default for CombiantionSet<T1, T2> {
	fn default() -> CombiantionSet<T1, T2> { CombiantionSet::new() }
}

impl<T1: PartialEq + Copy, T2: PartialEq + Copy> CombiantionSet<T1, T2> {
	pub fn new() -> CombiantionSet<T1, T2> {
		CombiantionSet {
			set: Vec::new(),
		}
	}

	pub fn insert(
		&mut self, first: T1, second: T2
	) {
		self.set.push((first, second));
	} 

	pub fn get_second_by_first(
		&self, first: &T1
	) -> Option<T2> {

		for z in &self.set {
			if z.0 == *first {
				return Some(z.1);
			}
		}

		None
	}

	pub fn remove_by_second(
		&mut self, second: &T2
	) -> DDSError {
		for i in range(0, self.set.len()) {
			if self.set[i].1 == *second {
				self.set.remove(i);
				return DDSError::Ok;
			}
		}
		DDSError::BadParameter
	}

	pub fn get_first_by_second(
		&self, second: &T2
	) -> Option<T1> {
		for z in &self.set {
			if z.1 == *second {
				return Some(z.0);
			}
		}

		None
	}
}