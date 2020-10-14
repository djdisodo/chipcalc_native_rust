use bit_reverse::LookupReverse;
use std::mem::replace;
use num_derive::FromPrimitive;

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix {
	pub x_size: u8,
	pub raw_map: Vec<u8>
}

impl Matrix {
	pub fn rotate(&self) -> Rotation {
		Rotation {
			matrix: self
		}
	}

	pub fn shr(&mut self, rhs: u8) {
		for i in 0..self.raw_map.len() as usize {
			self.raw_map[i] >>= rhs;
		}
	}

	pub fn shl(&mut self, rhs: u8) {
		for i in 0..self.raw_map.len() as usize {
			self.raw_map[i] <<= rhs;
		}
	}
}

pub struct Rotation<'a> {
	matrix: &'a Matrix
}

impl Rotation<'_> {

	pub fn get(&self, rotation: &MatrixRotation) -> Matrix {
		match rotation {
			MatrixRotation::Cw0 => self.matrix.clone(),
			MatrixRotation::Cw90 => self.cw90(),
			MatrixRotation::Cw180 => self.cw180(),
			MatrixRotation::Cw270 => self.cw270()
		}
	}

	fn cw90(&self) -> Matrix {
		let new_size = self.matrix.raw_map.len();
		let mut new_map = vec![0; self.matrix.x_size as usize];
		for i in 0..self.matrix.x_size as u8 {
			let bit_reader = 0b10000000 >> i;
			let mut bits = 0b0;
			for j in 0..self.matrix.raw_map.len() as usize {
				let a = (self.matrix.raw_map[j] & bit_reader) >> (7 - i) << 7;
				bits |= a;
				bits >>= 1;
			}
			new_map[i as usize] = bits << 1;
		}
		Matrix {
			x_size: new_size as u8,
			raw_map: new_map
		}
	}

	fn cw180(&self) -> Matrix {
		let mut new_map = self.matrix.raw_map.clone();
		new_map.reverse();
		for i in 0..self.matrix.raw_map.len() {
			new_map[i] = new_map[i].swap_bits();
			new_map[i] <<= 8 - self.matrix.x_size;
		}
		Matrix {
			x_size: self.matrix.x_size.clone(),
			raw_map: new_map
		}
	}

	fn cw270(&self) -> Matrix {
		self.cw90().rotate().cw180()
	}

	pub fn cache(&self) -> MatrixRotationCache {
		MatrixRotationCache::new(self)
	}
}

#[derive(Clone)]
pub struct MatrixRotationCache {
	pub cw0: Matrix,
	pub cw90: Matrix,
	pub cw180: Matrix,
	pub cw270: Matrix,
}

impl MatrixRotationCache {
	pub fn new(rotation: &Rotation) -> Self {
		Self {
			cw0: rotation.matrix.clone(),
			cw90: rotation.cw90(),
			cw180: rotation.cw180(),
			cw270: rotation.cw270()
		}
	}
	pub fn get(&self, rotation: &MatrixRotation) -> &Matrix {
		match rotation {
			MatrixRotation::Cw0 => &self.cw0,
			MatrixRotation::Cw90 => &self.cw90,
			MatrixRotation::Cw180 => &self.cw180,
			MatrixRotation::Cw270 => &self.cw270
		}
	}

	pub fn get_mut(&mut self, rotation: &MatrixRotation) -> &mut Matrix {
		match rotation {
			MatrixRotation::Cw0 => &mut self.cw0,
			MatrixRotation::Cw90 => &mut self.cw90,
			MatrixRotation::Cw180 => &mut self.cw180,
			MatrixRotation::Cw270 => &mut self.cw270
		}
	}
}

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum MatrixRotation {
	Cw0 = 0,
	Cw90 = 1,
	Cw180 = 2,
	Cw270 = 3
}

impl MatrixRotation {
	pub fn rotate_cw90(&mut self) {
		let a = *self;
		replace(self, match a {
			MatrixRotation::Cw0 => MatrixRotation::Cw90,
			MatrixRotation::Cw90 => MatrixRotation::Cw180,
			MatrixRotation::Cw180 => MatrixRotation::Cw270,
			MatrixRotation::Cw270 => MatrixRotation::Cw0
		});

	}
}