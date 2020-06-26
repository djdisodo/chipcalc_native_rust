#![feature(fn_traits)]

use crate::chip::{Chip, ChipRotationCache};
use crate::vector2::Vector2;
use crate::canvas::Canvas;
use crate::calculation::CalculationJob;
use std::path::Prefix::Verbatim;
use std::collections::VecDeque;

pub mod vector2;
pub mod canvas;
pub mod chip;
pub mod calculation;
pub mod num_queue;

fn main() {
	let chips = vec![
		Chip {
			size: Vector2::new(4, 3),
			raw_map: vec![
				0b11110000,
				0b10000000,
				0b10000000
			]
		},
		Chip {
			size: Vector2::new(1, 2),
			raw_map: vec![
				0b10000000,
				0b10000000
			]
		},
		Chip {
			size: Vector2::new(1, 2),
			raw_map: vec![
				0b10000000,
				0b10000000
			]
		}
	];

	let chips_rotation_cache: Vec<_> = chips.iter().map(| chip | chip.rotate().cache()).collect();
	let chips_rotation_cache_reference: Vec<_> = chips_rotation_cache.iter().map(| cache | cache).collect();
	let chips_rotation_cache_queue = VecDeque::from(chips_rotation_cache_reference.clone());

	let canvas = Canvas {
		size: Vector2::new(4, 3),
		raw_map: vec![
			0b10000000,
			0b10000000,
			0b00000000
		]
	};
	let mut job = CalculationJob::new(canvas, chips_rotation_cache_queue, Vec::new());
	let mut jobs = job.generate_jobs();
	let mut results = Vec::new();

	for x in &jobs {
		results.append(&mut x.calculate(0));
	}

	results.sort_by(| x, y | {
		x.len().cmp(&y.len())
	});

	for x in jobs {
		for x in x.calculate(0) {
			println!("----");
			for x in x {
				let no = chips_rotation_cache_reference.iter().position(| v | *v as *const ChipRotationCache == x.0);
				println!("no {} pos {} {} rot {:?}", no.unwrap(), x.1.x, x.1.y, x.2);
			}
		}
	}

}
