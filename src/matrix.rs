use crate::vector2::Vector2;
use bit_reverse::LookupReverse;
use std::ops::Shr;

#[derive(Clone)]
pub struct Matrix {
	pub size: Vector2<u8>,
	pub raw_map: Vec<u8>
}

impl Matrix {
	pub fn rotate(&self) -> Rotation {
		Rotation {
			chip: self
		}
	}

	pub fn shr(&mut self, rhs: u8) {
		for i in 0..self.size.y as usize {
			self.raw_map[i] >>= rhs;
		}
	}

	pub fn shl(&mut self, rhs: u8) {
		for i in 0..self.size.y as usize {
			self.raw_map[i] <<= rhs;
		}
	}
}

pub struct Rotation<'a> {
	chip: &'a Matrix
}

impl Rotation<'_> {

	pub fn cw90(&self) -> Matrix {
		let mut new_size = Vector2::<u8>::new(self.chip.size.y, self.chip.size.x);
		let mut new_map = vec![0; new_size.y as usize];
		for i in 0..self.chip.size.x as u8 {
			let bit_reader = 0b10000000 >> i;
			let mut bits = 0b0;
			for j in 0..self.chip.size.y as usize {
				let a = (self.chip.raw_map[j] & bit_reader) >> (7 - i) << 7;
				bits |= a;
				bits >>= 1;
			}
			new_map[i as usize] = bits << 1;
		}
		Matrix {
			size: new_size,
			raw_map: new_map
		}
	}

	pub fn cw180(&self) -> Matrix {
		let mut new_map = self.chip.raw_map.clone();
		new_map.reverse();
		for i in 0..self.chip.size.y as usize {
			new_map[i] = new_map[i].swap_bits();
			new_map[i] <<= 8 - self.chip.size.x;
		}
		Matrix {
			size: self.chip.size.clone(),
			raw_map: new_map
		}
	}

	pub fn cw270(&self) -> Matrix {
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
			cw0: rotation.chip.clone(),
			cw90: rotation.cw90(),
			cw180: rotation.cw180(),
			cw270: rotation.cw270()
		}
	}
}