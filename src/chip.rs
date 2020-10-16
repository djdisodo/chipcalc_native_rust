use crate::matrix::{MatrixRotationCache, MatrixRotation};
use crate::shape::Shape;
use std::ops::Deref;
use num_rational::Rational32;
use crate::stat::Stat;
use num_derive::FromPrimitive;
use serde_json::Value;
use num_traits::cast::FromPrimitive;

pub struct Chip {
	pub id: u32,
	pub shape: Shape,
	pub color: Color,
	pub pt: Stat,
	pub rank: usize,
	pub level: i32,
	pub rotation: MatrixRotation
}

impl Chip {
	pub const RATE_DMG: Rational32 = Rational32::new_raw(44, 10);
    pub const RATE_BRK: Rational32 = Rational32::new_raw(127, 10);
    pub const RATE_HIT: Rational32 = Rational32::new_raw(71, 10);
    pub const RATE_RLD: Rational32 = Rational32::new_raw(57, 10);


	pub fn new(
		id: u32,
		shape: Shape,
		color: Color,
		pt: Stat,
		rank: usize,
		level: i32,
		rotation: MatrixRotation
	) -> Self {
		Self {
			id,
			shape,
			color,
			pt,
			rank,
			level,
			rotation
		}
	}

	pub fn from_json(value: Value) -> Option<Self> {
		let id: u32 = value["id"].as_str().unwrap().parse().unwrap();
		let mut shape_info = value["shape_info"].as_str().unwrap().split(",");
		let rotation = MatrixRotation::from_u32(shape_info.next().unwrap().parse().unwrap()).unwrap();
		let dmg: i32 = value["assist_damage"].as_i64().unwrap() as i32;
		let brk: i32 = value["assist_def_break"].as_i64().unwrap() as i32;
		let hit: i32 = value["assist_hit"].as_i64().unwrap() as i32;
		let rld: i32 = value["assist_reload"].as_i64().unwrap() as i32;
		let pt = Stat::new(dmg, brk, hit, rld);
		let rank: usize = value["chip_id"].as_str().unwrap()[0..1].parse().unwrap();
		let level: i32 = value["chip_level"].as_i64().unwrap() as i32;
		let color = Color::from_i64(value["color_id"].as_i64().unwrap()).unwrap();
		let shape: Shape = Shape::from_u32(value["grid_id"].as_str().unwrap().parse().unwrap()).unwrap();
		Some(Self {
			id,
			shape,
			color,
			pt,
			rank,
			level,
			rotation
		})
	}

	pub fn get_rotation(&self) -> &MatrixRotation {
		&self.rotation
	}

	pub fn get_stat(&self) -> Stat {
		Stat::new(
			self.__get_stat(Self::RATE_DMG, self.pt.dmg),
			self.__get_stat(Self::RATE_BRK, self.pt.brk),
			self.__get_stat(Self::RATE_HIT, self.pt.hit),
			self.__get_stat(Self::RATE_RLD, self.pt.rld)
		)
	}


	fn __get_stat(&self, rate: Rational32, pt: i32) -> i32 {
		let base = (rate * self.get_type().get_multiplier(self.rank as i32) * pt).ceil();
		(self.get_level_multiplier() * base).ceil().to_integer()
	}

	pub fn get_level_multiplier(&self) -> Rational32 {
		if self.level < 10 {
			Rational32::new_raw(self.level, 1) * 8 / 100 + 1
		} else {
			Rational32::new_raw(self.level, 1) * 7 / 100 + Rational32::new_raw(11, 10)
		}
	}

	pub fn get_correction_cost(&self) -> usize {
		self.rank * 10
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