use crate::matrix::{MatrixRotationCache, MatrixRotation};
use crate::shape::Shape;
use std::ops::Deref;

pub struct Chip {
	shape: Shape,
	initial_rotation: MatrixRotation,
	//todo stats
}

impl Chip {
	pub fn new(shape: Shape, initial_rotation: MatrixRotation) -> Self {
		Self {
			shape,
			initial_rotation
		}
	}

	pub fn get_initial_rotation(&self) -> &MatrixRotation {
		&self.initial_rotation
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