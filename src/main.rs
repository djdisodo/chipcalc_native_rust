#![feature(fn_traits)]

use crate::chip::Chip;
use crate::vector2::Vector2;
use crate::canvas::Canvas;
use crate::calculation::CalculationJob;
use std::path::Prefix::Verbatim;

pub mod vector2;
pub mod canvas;
pub mod chip;
pub mod calculation;
pub mod num_queue;

fn main() {
	let map = vec![
		0b11100000,
		0b10000000,
		0b10000000
	];
	let canvas = Canvas {
		size: Vector2::new(2, 3),
		raw_map: vec![
			0b10000000,
			0b10000000,
			0b00000000
		]
	};
	let chip = Chip {
		size: Vector2::new(2, 3),
		raw_map: map
	};
}
