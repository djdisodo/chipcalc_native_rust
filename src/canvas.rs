use crate::vector2::Vector2;

#[derive(Clone)]
pub struct Canvas {
	pub size: Vector2<u8>,
	pub raw_map: Vec<u8>
}

impl Canvas {
	pub fn get_left_space(&self) -> u8 {
		let mut left_space: u8 = 0;
		for x in &self.raw_map {
			for i in 0..8 {
				left_space += ((!x >> i) & 0x00000001);
			}
		}
		left_space
	}

	pub fn mvl(&mut self) {
		for i in 0..self.raw_map.len() as usize {
			self.raw_map[i] <<= 1;
			self.raw_map[i] |= 0b1;
		}
	}
}