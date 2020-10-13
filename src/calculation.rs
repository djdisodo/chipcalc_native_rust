use crate::canvas::Canvas;
use crate::matrix::{MatrixRotationCache, Matrix};
use crate::vector2::Vector2;
use crate::matrix::MatrixRotation::{Cw0, Cw180, Cw90, Cw270};
use crate::matrix::MatrixRotation;
use std::collections::VecDeque;
use crate::chip::Chip;
use crate::shape::Shape;

pub struct CalculationJob<'a> {
	canvas: Canvas,
	all_chips: &'a Vec<Chip>,
	chips: VecDeque<usize>,
	base: Vec<(usize, Vector2<u8>, MatrixRotation)>,
	config: Config
}

impl <'a> CalculationJob<'a> {
	pub fn new(
		canvas: Canvas,
		all_chips: &'a Vec<Chip>,
		chips: VecDeque<usize>,
		base: Vec<(usize, Vector2<u8>, MatrixRotation)>,
		config: Config
	) -> Self {
		Self {
			canvas,
			all_chips,
			chips,
			base,
			config
		}
	}

	pub fn generate_jobs(&self) -> GenerateJob {
		GenerateJob::new(self)
	}

	pub fn calculate(&self) -> Vec<Vec<(usize, Vector2<u8>, MatrixRotation)>> {
		let result = calculate(&self.canvas, self.all_chips, &self.chips, &self.base, &self.config);
		return if result.is_some() {
			result.unwrap()
		} else {
			let mut result = Vec::new();
			result.push(self.base.clone());
			result
		}
	}
}

pub struct GenerateJob<'a> {
	job: &'a CalculationJob<'a>,
	chips: VecDeque<usize>,
	chips_base: VecDeque<usize>,
	cache: VecDeque<CalculationJob<'a>>
}

impl <'a> GenerateJob<'a> {
	pub fn new(job: &'a CalculationJob) -> Self {
		Self {
			job,
			chips: job.chips.clone(),
			chips_base: job.chips.clone(),
			cache: VecDeque::with_capacity(4)
		}
	}
}

impl <'a> Iterator for GenerateJob<'a> {
	type Item = CalculationJob<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.cache.is_empty() {
			let chip = self.chips.pop_front();
			if chip.is_none() {
				return None;
			}
			let mut chips = self.chips_base.clone();
			chips.remove(chip.unwrap());
			let rotate = self.job.config.rotate;
			try_put(
				&self.job.canvas,
				self.job.all_chips.get(chip.unwrap()).unwrap(),
				| canvas, pos, rotation | {
					let mut base = self.job.base.clone();
					base.push((chip.unwrap(), pos, rotation));
					self.cache.push_back(
						CalculationJob::new(
							canvas,
							self.job.all_chips,
							chips.clone(),
							base,
							self.job.config
						),
					);
					true
				},
				rotate
			)
		}
		self.cache.pop_front()
	}
}

fn calculate<'a>(
	canvas: &Canvas,
	all_chips: &'a Vec<Chip>,
	chips: &VecDeque<usize>,
	base: &Vec<(usize, Vector2<u8>, MatrixRotation)>,
	config: &Config
) -> Option<Vec<Vec<(usize, Vector2<u8>, MatrixRotation)>>> {
	let mut result = Vec::new();
	if canvas.get_left_space() <= config.max_left_space {
		return None;
	}
	let mut chips = chips.clone();
	let mut chip = chips.pop_front();
	while chip.is_some() {
		try_put(
			canvas,
			all_chips.get(chip.unwrap()).unwrap(),
			| canvas, position, rotation | {
				let mut base = base.clone();
				base.push((chip.unwrap(), position.clone(), rotation.clone()));
				let r = calculate(&canvas, all_chips, &chips, &base, config);
				if r.is_some() {
					result.append(&mut r.unwrap());
				}
				canvas.get_left_space() != 0
			},
			config.rotate
		);
		chip = chips.pop_front();
	}
	if result.is_empty() {
		result.push(base.clone());
		return Some(result);
	}
	Some(result)
}

fn try_put(canvas: &Canvas, chip: &Chip, mut on_put: impl FnMut(Canvas, Vector2<u8>, MatrixRotation) -> bool, rotate: bool) {
	let mut matrix_rotation_cache = chip.get_rotation_cache().clone();
	let mut rot = *chip.get_initial_rotation();

	if rotate {
		for _ in 0..chip.get_max_rotation() {
			rot.rotate_cw90();
			__try_put(
				canvas,
				matrix_rotation_cache.get_mut(&rot),
				|canvas, pos | on_put.call_mut((canvas, pos, rot))
			);
		}
	}

}

fn __try_put(canvas: &Canvas, matrix: &mut Matrix, mut on_put: impl FnMut(Canvas, Vector2<u8>) -> bool) {
	for x in 0..canvas.size.x {
		if matrix.x_size + x > canvas.size.x {
			break;
		}
		for y in 0..canvas.size.y {
			if (matrix.raw_map.len() as u8) + y > canvas.size.y {
				break;
			}
			let mut new_canvas = canvas.clone();
			let mut fit = true;
			for i in 0..matrix.raw_map.len() {
				if new_canvas.raw_map[i + y as usize] & matrix.raw_map[i] == 0b0 {
					new_canvas.raw_map[i + y as usize] |= matrix.raw_map[i];
				} else {
					fit = false;
					break;
				}
			}
			if fit {
				let pos = Vector2::new(x, y);
				if !on_put.call_mut((new_canvas, pos)) {
					return;
				}
			}
		}
		matrix.shr(1);
	}
	return;
}



#[derive(Clone, Copy, Debug)]
pub struct Config {
	pub max_left_space: u8,
	pub rotate: bool
}

pub enum Board {
	NameBGM71,
	NameAGS30,
	Name2B14,
	NameM2,
	NameAT4,
	NameQLZ04,
	NameMk153
}

impl Board {
	pub fn to_canvas(&self, level: u8) -> Canvas {
		let map: [[u8; 8]; 8] = match self {
			Board::NameBGM71 => [
				[6, 6, 6, 6, 6, 6, 6, 6],
				[6, 4, 4, 4, 3, 3, 3, 6],
				[6, 4, 1, 1, 1, 1, 2, 6],
				[6, 2, 1, 1, 1, 1, 2, 6],
				[6, 2, 1, 1, 1, 1, 2, 6],
				[6, 2, 1, 1, 1, 1, 5, 6],
				[6, 3, 3, 3, 5, 5, 5, 6],
				[6, 6, 6, 6, 6, 6, 6, 6]
			],
			Board::NameAGS30 => [
				[6, 6, 5, 5, 6, 6, 6, 6],
				[6, 3, 3, 2, 2, 6, 6, 6],
				[4, 3, 1, 1, 1, 1, 6, 6],
				[4, 2, 1, 1, 1, 1, 2, 6],
				[6, 2, 1, 1, 1, 1, 2, 4],
				[6, 6, 1, 1, 1, 1, 3, 4],
				[6, 6, 6, 2, 2, 3, 3, 6],
				[6, 6, 6, 6, 5, 5, 6, 6]
			],
			Board::Name2B14 => [
				[6, 6, 6, 6, 6, 6, 6, 6],
				[6, 6, 5, 6, 6, 5, 6, 6],
				[6, 2, 1, 1, 1, 1, 3, 6],
				[4, 2, 1, 1, 1, 1, 3, 4],
				[4, 2, 1, 1, 1, 1, 3, 4],
				[6, 2, 1, 1, 1, 1, 3, 6],
				[6, 6, 5, 6, 6, 5, 6, 6],
				[6, 6, 6, 6, 6, 6, 6, 6]
			],
			Board::NameM2 => [
				[5, 3, 3, 6, 6, 6, 6, 5],
				[6, 3, 1, 1, 6, 6, 2, 4],
				[6, 6, 1, 1, 6, 2, 2, 4],
				[6, 6, 1, 1, 1, 1, 2, 6],
				[6, 2, 1, 1, 1, 1, 6, 6],
				[4, 2, 2, 6, 1, 1, 6, 6],
				[4, 2, 6, 6, 1, 1, 3, 6],
				[5, 6, 6, 6, 6, 3, 3, 5]
			],
			Board::NameAT4 => [
				[6, 6, 6, 1, 1, 6, 6, 6],
				[6, 6, 1, 1, 1, 1, 6, 6],
				[6, 1, 1, 1, 1, 1, 1, 6],
				[2, 1, 1, 6, 6, 1, 1, 3],
				[2, 2, 2, 6, 6, 3, 3, 3],
				[6, 2, 2, 4, 4, 3, 3, 6],
				[6, 6, 5, 4, 4, 5, 6, 6],
				[6, 6, 6, 5, 5, 6, 6, 6]
			],
			Board::NameQLZ04 => [
				[6, 6, 6, 6, 6, 6, 6, 6],
				[5, 3, 6, 6, 6, 6, 3, 5],
				[5, 3, 3, 6, 6, 3, 3, 5],
				[4, 1, 1, 1, 1, 1, 1, 4],
				[4, 1, 1, 1, 1, 1, 1, 4],
				[6, 1, 1, 2, 2, 1, 1, 6],
				[6, 6, 2, 2, 2, 2, 6, 6],
				[6, 6, 6, 2, 2, 6, 6, 6]
			],
			Board::NameMk153 => [
				[6, 6, 2, 2, 6, 6, 6, 6],
				[6, 6, 2, 2, 5, 5, 5, 6],
				[6, 6, 2, 2, 4, 4, 4, 6],
				[6, 6, 2, 2, 3, 3, 4, 6],
				[1, 1, 1, 1, 1, 1, 3, 3],
				[1, 1, 1, 1, 1, 1, 3, 3],
				[6, 5, 1, 1, 6, 6, 6, 6],
				[6, 6, 1, 1, 6, 6, 6, 6]
			]
		};

		let mut canvas_base = vec![0xffu8; 8];

		for y in 0..8 {
			let mut base = 0;
			for x in 0..8 {
				base <<= 1;
				if map[y][x] > level {
					base |= 1;
				} else {
					base |= 0;
				}
			}
			canvas_base[y] = base;
		}

		Canvas {
			size: Vector2::new(8, 8),
			raw_map: canvas_base
		}
	}
}