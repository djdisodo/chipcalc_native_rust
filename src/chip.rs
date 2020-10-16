use crate::matrix::{MatrixRotationCache, MatrixRotation};
use crate::shape::Shape;
use std::ops::Deref;
use num_rational::Rational32;
use crate::stat::Stat;

pub struct Chip {
	shape: Shape,
	initial_rotation: MatrixRotation,
	star: usize,
	//todo stats
}

impl Chip {
	pub const RATE_DMG: Rational32 = Rational32::new_raw(44, 10);
    pub const RATE_BRK: Rational32 = Rational32::new_raw(127, 10);
    pub const RATE_HIT: Rational32 = Rational32::new_raw(71, 10);
    pub const RATE_RLD: Rational32 = Rational32::new_raw(57, 10);


	pub fn new(shape: Shape, initial_rotation: MatrixRotation) -> Self {
		Self {
			shape,
			initial_rotation
		}
	}

	pub fn get_initial_rotation(&self) -> &MatrixRotation {
		&self.initial_rotation
	}

	pub fn get_stat(&self) -> Stat {
		Stat::new(
			self.__get_stat(Self::RATE_DMG, self.pt.RATE_DMG),
			self.__get_stat(Self::RATE_BRK, self.pt.RATE_BRK),
			self.__get_stat(Self::RATE_HIT, self.pt.RATE_HIT),
			self.__get_stat(Self::RATE_RLD, self.pt.RATE_RLD)
		);
	}


	fn __get_stat(&self, rate: Rational32, pt: i32) -> u32 {
		let base = (rate * self.get_type().get_multiplier(self.star as i32) * pt) as i32;
		
	}
}

impl Deref for Chip {
	type Target = Shape;

	fn deref(&self) -> &Self::Target {
		&self.shape
	}
}

#[derive(FromPrimitive, Clone, Copy)]
pub enum Color {
	Orange = 1,
	Blue = 2
}