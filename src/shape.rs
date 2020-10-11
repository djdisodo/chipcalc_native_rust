use std::cmp::Ordering;
use num_derive::FromPrimitive;
use crate::matrix::Matrix;
use crate::vector2::Vector2;

#[allow(non_pascal_case)]
#[derive(FromPrimitive, Clone, Copy)]
pub enum Shape {
	NONE = 0,
	// 1 = A
	_1 = 1,
	// 2 = B
	_2 = 2,
	// 3 = C
	_3_I = 3, _3_L = 4,
	// 4 = D
	_4_I = 5, _4_O = 6, _4_Lm = 7, _4_L = 8, _4_Zm = 9, _4_Z = 10, _4_T = 11,
	// 5A = E
	_5A_Pm = 12, _5A_P = 13, _5A_I = 14, _5A_C = 15, _5A_Z = 16, _5A_Zm = 17, _5A_V = 18, _5A_L = 19, _5A_Lm = 20,
	// 5B = F
	_5B_W = 21, _5B_Nm = 22, _5B_N = 23, _5B_Ym = 24, _5B_Y = 25, _5B_X = 26, _5B_T = 27, _5B_F = 28, _5B_Fm = 29,
	// 6 = G
	_6_O = 30, _6_A = 31, _6_D = 32, _6_Z = 33, _6_Zm = 34, _6_Y = 35, _6_T = 36, _6_I = 37, _6_C = 38, _6_R = 39
}
const DEFAULT: Shape = Shape::_1;

#[derive(FromPrimitive, Clone, Copy)]
pub enum Type {
	NONE = 0, _1 = 1, _2 = 2, _3 = 3, _4 = 4, _5A = 5, _5B = 6, _6 = 7
}

impl Type {
	pub fn by_name(name: &str) -> Type {
		match name {
			"6"=> Type::_6,
			"5B" | "5b"=> Type::_5B,
			"5A" | "5a"=> Type::_5A,
			"4"=> Type::_4,
			"3"=> Type::_3,
			"2"=> Type::_2,
			"1"=> Type::_1,
			_ => Type::NONE,
		}
	}

}

impl PartialEq for Type {
	fn eq(&self, other: &Self) -> bool {
		self == other
	}
}

impl PartialOrd for Type {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some((*self as u8).cmp(&(*other as u8)))
	}
}


impl Shape {
	pub fn get_size(&self) -> usize {
		let id: usize = *self as usize;
		id - if 6 <= id { 1 } else { 0 }
	}

	const SHAPE_1: &'static [Shape] = &[Shape::_1];
	const SHAPE_2: &'static [Shape] = &[Shape::_2];
	const SHAPE_3: &'static [Shape] = &[Shape::_3_I, Shape::_3_L];
	const SHAPE_4: &'static [Shape] = &[Shape::_4_I, Shape::_4_O, Shape::_4_Lm, Shape::_4_L, Shape::_4_Zm, Shape::_4_Z, Shape::_4_T];
	const SHAPE_5A: &'static [Shape] = &[Shape::_5A_Pm, Shape::_5A_P, Shape::_5A_I, Shape::_5A_C, Shape::_5A_Z, Shape::_5A_Zm, Shape::_5A_V, Shape::_5A_L, Shape::_5A_Lm];
	const SHAPE_5B: &'static [Shape] = &[Shape::_5B_W, Shape::_5B_Nm, Shape::_5B_N, Shape::_5B_Ym, Shape::_5B_Y, Shape::_5B_X, Shape::_5B_T, Shape::_5B_F, Shape::_5B_Fm];
	const SHAPE_5: &'static [Shape] = &[
		Shape::_5A_Pm, Shape::_5A_P, Shape::_5A_I, Shape::_5A_C, Shape::_5A_Z, Shape::_5A_Zm, Shape::_5A_V, Shape::_5A_L, Shape::_5A_Lm,
		Shape::_5B_W, Shape::_5B_Nm, Shape::_5B_N, Shape::_5B_Ym, Shape::_5B_Y, Shape::_5B_X, Shape::_5B_T, Shape::_5B_F, Shape::_5B_Fm
	];
	const SHAPE_6: &'static [Shape] = &[Shape::_6_O, Shape::_6_A, Shape::_6_D, Shape::_6_Z, Shape::_6_Zm, Shape::_6_Y, Shape::_6_T, Shape::_6_I, Shape::_6_C, Shape::_6_R];

	pub fn by_name(name: &str) -> Shape {
		match name {
			// 1 = A
			"1" => Shape::_1,
			// 2 = B
			"2" => Shape::_2,
			// 3 = C
			"3I" => Shape::_3_I,
			"3L" => Shape::_3_L,
			// 4 = D
			"4I" => Shape::_4_I,
			"4O" => Shape::_4_O,
			"4Lm" => Shape::_4_Lm,
			"4L" => Shape::_4_L,
			"4Zm" => Shape::_4_Zm,
			"4Z" => Shape::_4_Z,
			"4T" => Shape::_4_T,
			// 5A = E
			"5Pm" => Shape::_5A_Pm,
			"5P" => Shape::_5A_P,
			"5I" => Shape::_5A_I,
			"5C" => Shape::_5A_C,
			"5Z" => Shape::_5A_Z,
			"5Zm" => Shape::_5A_Zm,
			"5V" => Shape::_5A_V,
			"5L" => Shape::_5A_L,
			"5Lm" => Shape::_5A_Lm,
			// 5B = F
			"5W" => Shape::_5B_W,
			"5Nm" => Shape::_5B_Nm,
			"5N" => Shape::_5B_N,
			"5Ym" => Shape::_5B_Ym,
			"5Y" => Shape::_5B_Y,
			"5X" => Shape::_5B_X,
			"5T" => Shape::_5B_T,
			"5F" => Shape::_5B_F,
			"5Fm" => Shape::_5B_Fm,
			// 6 = G
			"6O" => Shape::_6_O,
			"6A" => Shape::_6_A,
			"6D" => Shape::_6_D,
			"6Z" => Shape::_6_Z,
			"6Zm" => Shape::_6_Zm,
			"6Y" => Shape::_6_Y,
			"6T" => Shape::_6_T,
			"6I" => Shape::_6_I,
			"6C" => Shape::_6_C,
			"6R" => Shape::_6_R,
			_ => Shape::NONE,
		}
	}

	pub fn to_matrix(&self) -> Matrix {
		match self {
			Shape::_1 => Matrix {
				size: Vector2::new(1, 1),
				raw_map: vec![
					0b10000000
				]
			},
			Shape::_2 => Matrix {
				size: Vector2::new(1, 2),
				raw_map: vec![
					0b10000000,
					0b10000000
				]
			},
			Shape::_3_I => Matrix {
				size: Vector2::new(1, 3),
				raw_map: vec![
					0b10000000,
					0b10000000,
					0b10000000
				]
			},
			Shape::_3_L => Matrix {
				size: Vector2::new(2, 2),
				raw_map: vec![
					0b10000000,
					0b11000000
				]
			},
			Shape::_4_I => Matrix {
				size: Vector2::new(4, 1),
				raw_map: vec![
					0b11110000
				]
			},
			Shape::_4_L => Matrix {
				size: Vector2::new(2, 3),
				raw_map: vec![
					0b10000000,
					0b10000000,
					0b11000000
				]
			},
			Shape::_4_Lm => Matrix {
				size: Vector2::new(3, 2),
				raw_map: vec![
					0b10000000,
					0b11100000
				]
			},
			Shape::_4_O => Matrix {
				size: Vector2::new(2, 2),
				raw_map: vec![
					0b11000000,
					0b11000000
				]
			},
			Shape::_4_T => Matrix {
				size: Vector2::new(3, 2),
				raw_map: vec![
					0b01000000,
					0b11100000
				]
			},
			Shape::_4_Z => Matrix {
				size: Vector2::new(3, 2),
				raw_map: vec![
					0b11000000,
					0b01100000,
				]
			},
			Shape::_4_Zm => Matrix {
				size: Vector2::new(3, 2),
				raw_map: vec![
					0b01100000,
					0b11000000
				]
			},
			Shape::_5A_C => Matrix {
				size: Vector2::new(3, 2),
				raw_map: vec![
					0b11100000,
					0b10100000
				]
			},
			Shape::_5A_I => Matrix {
				size: Vector2::new(5, 1),
				raw_map: vec![
					0b11111000
				]
			},
			Shape::_5A_L => Matrix {
				size: Vector2::new(2, 4),
				raw_map: vec![
					0b10000000,
					0b10000000,
					0b10000000,
					0b11000000
				]
			},
			Shape::_5A_Lm => Matrix {
				size: Vector2::new(2, 4),
				raw_map: vec![
					0b01000000,
					0b01000000,
					0b01000000,
					0b11000000
				]
			},
			Shape::_5A_P => Matrix {
				size: Vector2::new(2, 3),
				raw_map: vec![
					0b01000000,
					0b11000000,
					0b11000000
				]
			},
			Shape::_5A_Pm => Matrix {
				size: Vector2::new(2, 3),
				raw_map: vec![
					0b10000000,
					0b11000000,
					0b11000000
				]
			},
			Shape::_5A_V => Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b10000000,
					0b10000000,
					0b11100000
				]
			},
			Shape::_5A_Z =>  Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b00100000,
					0b11100000,
					0b10000000
				]
			},
			Shape::_5A_Zm => Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b10000000,
					0b11100000,
					0b00100000
				]
			},
			Shape::_5B_F => Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b10000000,
					0b11100000,
					0b01000000
				]
			},
			Shape::_5B_Fm => Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b00100000,
					0b11100000,
					0b01000000
				]
			},
			Shape::_5B_N => Matrix {
				size: Vector2::new(2, 4),
				raw_map: vec![
					0b01000000,
					0b11000000,
					0b10000000,
					0b10000000
				]
			},
			Shape::_5B_Nm => Matrix {
				size: Vector2::new(2, 4),
				raw_map: vec![
					0b10000000,
					0b11000000,
					0b01000000,
					0b01000000
				]
			},
			Shape::_5B_T => Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b01000000,
					0b01000000,
					0b11100000
				]
			},
			Shape::_5B_W => Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b01100000,
					0b11000000,
					0b10000000
				]
			},
			Shape::_5B_X => Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b01000000,
					0b11100000,
					0b01000000
				]
			},
			Shape::_5B_Y => Matrix {
				size: Vector2::new(2, 4),
				raw_map: vec![
					0b01000000,
					0b11000000,
					0b01000000,
					0b01000000
				]
			},
			Shape::_5B_Ym => Matrix {
				size: Vector2::new(2, 4),
				raw_map: vec![
					0b10000000,
					0b11000000,
					0b10000000,
					0b10000000
				]
			},
			Shape::_6_A => Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b10000000,
					0b11000000,
					0b11100000
				]
			},
			Shape::_6_C => Matrix {
				size: Vector2::new(4, 2),
				raw_map: vec![
					0b10010000,
					0b11110000
				]
			},
			Shape::_6_D => Matrix {
				size: Vector2::new(4, 2),
				raw_map: vec![
					0b01100000,
					0b11110000
				]
			},
			Shape::_6_I => Matrix {
				size: Vector2::new(6, 1),
				raw_map: vec![
					0b11111100
				]
			},
			Shape::_6_O => Matrix {
				size: Vector2::new(2, 3),
				raw_map: vec![
					0b11000000,
					0b11000000,
					0b11000000
				]
			},
			Shape::_6_R => Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b01000000,
					0b11100000,
					0b11000000
				]
			},
			Shape::_6_T => Matrix {
				size: Vector2::new(4, 3),
				raw_map: vec![
					0b00100000,
					0b11110000,
					0b00100000
				]
			},
			Shape::_6_Y => Matrix {
				size: Vector2::new(3, 3),
				raw_map: vec![
					0b01000000,
					0b11100000,
					0b10100000
				]
			},
			Shape::_6_Z => Matrix {
				size: Vector2::new(4, 2),
				raw_map: vec![
					0b11100000,
					0b01110000
				]
			},
			Shape::_6_Zm => Matrix {
				size: Vector2::new(4, 2),
				raw_map: vec![
					0b01110000,
					0b11100000
				]
			},
			Shape::NONE => panic!()
		}
	}
}