#![feature(fn_traits)]

use crate::matrix::{Matrix, MatrixRotationCache};
use crate::vector2::Vector2;
use crate::canvas::Canvas;
use crate::calculation::CalculationJob;
use std::path::Prefix::Verbatim;
use std::collections::VecDeque;
use crate::calculation::Config;

pub mod vector2;
pub mod canvas;
pub mod matrix;
pub mod calculation;
pub mod num_queue;

fn main() {
	let chips = vec![
		Matrix {
			size: Vector2::new(4, 3),
			raw_map: vec![
				0b11110000,
				0b10000000,
				0b10000000
			]
		},
		Matrix {
			size: Vector2::new(1, 2),
			raw_map: vec![
				0b10000000,
				0b10000000
			]
		},
		Matrix {
			size: Vector2::new(1, 2),
			raw_map: vec![
				0b10000000,
				0b10000000
			]
		}
	];

	let chips_rotation_cache: Vec<_> = chips.iter().map(| chip | chip.rotate().cache()).collect();
	let chips_no: Vec<usize> = (0..chips_rotation_cache.len()).collect();
	let chips_no_queue: VecDeque<usize> = VecDeque::from(chips_no);

	let canvas = Canvas {
		size: Vector2::new(4, 3),
		raw_map: vec![
			0b10000000,
			0b10000000,
			0b00000000
		]
	};
	let mut job = CalculationJob::new(canvas, &chips_rotation_cache, chips_no_queue, Vec::new(), Config { max_left_space: 10, rotate: false });
	let mut jobs = job.generate_jobs();
	let mut results = Vec::new();

	for x in jobs {
		results.append(&mut x.calculate(0));
	}

	results.sort_by(| x, y | {
		x.len().cmp(&y.len())
	});

	for x in results {
		println!("----");
		for x in x {
			println!("no {} pos {} {} rot {:?}", x.0, x.1.x, x.1.y, x.2);
		}
	}

}
