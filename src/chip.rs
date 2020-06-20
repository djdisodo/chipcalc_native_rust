use crate::vector2::Vector2;
use bit_reverse::LookupReverse;
use std::ops::Shr;

#[derive(Clone)]
pub struct Chip {
	pub size: Vector2<u8>,
	pub raw_map: Vec<u8>
}

impl Chip {
	pub fn rotate(&self) -> Rotation {
		Rotation {
			chip: self
		}
	}

	pub fn shr(mut self, rhs: u8) -> Self {
		for i in 0..self.size.y as usize {
			self.raw_map[i] >>= rhs;
		}
		return self
	}

	pub fn shl(mut self, rhs: u8) -> Self {
		for i in 0..self.size.y as usize {
			self.raw_map[i] <<= rhs;
		}
		return self
	}
}

pub struct Rotation<'a> {
	chip: &'a Chip
}

impl Rotation<'_> {

	pub fn cw90(&self) -> Chip {
		let mut new_size = Vector2::<u8>::new(self.chip.size.y, self.chip.size.x);
		let mut new_map = Vec::<u8>::with_capacity(new_size.y as usize);
		for i in 0..self.chip.size.x as u8 {
			let bit_reader = 0b1 << i;
			let mut bits = 0b0;
			for j in (0..self.chip.size.y as usize).rev() {
				bits &= (self.chip.raw_map[j] & bit_reader) >> i;
				bits <<= 1;
			}
			new_map[i as usize] = bits;
		}
		Chip {
			size: new_size,
			raw_map: new_map
		}
	}

	pub fn cw180(&self) -> Chip {
		let mut new_map = self.chip.raw_map.clone();
		new_map.reverse();
		for i in 0..self.chip.size.y as usize {
			new_map[i] = new_map[i].swap_bits();
			new_map[i] <<= 8 - self.chip.size.x;
		}
		Chip {
			size: self.chip.size.clone(),
			raw_map: new_map
		}
	}

	pub fn cw270(&self) -> Chip {
		self.cw90().rotate().cw180()
	}
}

pub struct ChipRotationCache {
	pub cw0: Chip,
	pub cw90: Chip,
	pub cw180: Chip,
	pub cw270: Chip,
}

impl ChipRotationCache {
	pub fn new(rotation: &Rotation) -> Self {
		Self {
			cw0: rotation.chip.clone(),
			cw90: rotation.cw90(),
			cw180: rotation.cw180(),
			cw270: rotation.cw270()
		}
	}
}