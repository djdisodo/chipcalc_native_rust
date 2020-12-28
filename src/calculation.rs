use crate::canvas::Canvas;
use crate::matrix::{MatrixRotationCache, Matrix};
use crate::vector2::Vector2;
use crate::matrix::MatrixRotation::{Cw0, Cw180, Cw90, Cw270};
use crate::matrix::MatrixRotation;
use std::collections::VecDeque;
use crate::chip::Chip;
use crate::shape::Shape;
use strum_macros::EnumString;
use std::ops::{Deref, DerefMut, Range};
use crate::stat::Stat;

pub struct CalculationJob<'a> {
	canvas: Canvas,
	all_chips: &'a Vec<Chip>,
	base: CalculationResult,
	config: Config
}

impl <'a> CalculationJob<'a> {
	pub fn new(
		canvas: Canvas,
		all_chips: &'a Vec<Chip>,
		base: CalculationResult,
		config: Config
	) -> Self {
		Self {
			canvas,
			all_chips,
			base,
			config
		}
	}

	pub fn generate_jobs(self) -> GenerateJob<'a> {
		GenerateJob::new(self)
	}

	pub fn calculate<F: FnMut(CalculationResult)>(&self, on_found: &mut F) {
		calculate(&self.canvas, self.all_chips, &self.base, on_found, &self.config)
	}
}

pub struct GenerateJob<'a> {
	job: CalculationJob<'a>,
	chips: Range<usize>,
	cache: VecDeque<CalculationJob<'a>>
}

impl <'a> GenerateJob<'a> {
	pub fn new(job: CalculationJob<'a>) -> Self {
		let chips = 0..job.all_chips.len();
		Self {
			job,
			chips,
			cache: VecDeque::with_capacity(4)
		}
	}
}

impl <'a> Iterator for GenerateJob<'a> {
	type Item = CalculationJob<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.cache.is_empty() {
			if let Some(chip_index) = self.chips.next() {
				let chip = &self.job.all_chips[chip_index];
				if !self.job.base.is_used(chip_index) {
					let cache = &mut self.cache;
					let job = &self.job;
					try_put(
						&self.job.canvas,
						chip,
						&mut | canvas, pos, rotation | {
							let mut base = job.base.clone();
							base.push(CalculationResultChip {
								chip_index,
								position: pos,
								rotation
							});
							if chip.rotation != rotation {
								base.correction_cost += chip.get_correction_cost();
							}
							cache.push_back(
								CalculationJob::new(
									canvas,
									job.all_chips,
									base,
									job.config
								),
							);
						},
						&self.job.config
					)
				}
			} else {
				return None;
			}

		}
		self.cache.pop_front()
	}
}

#[inline(always)]
fn calculate<'a, F: FnMut(CalculationResult)>(
	canvas: &Canvas,
	all_chips: &'a Vec<Chip>,
	base: &CalculationResult,
	on_found: &mut F,
	config: &Config
) {
	for chip_index in 0..all_chips.len() {
		if base.is_used(chip_index) {
			continue;
		}
		let chip = &all_chips[chip_index];
		try_put(
			canvas,
			chip,
			&mut | canvas, position, rotation | {
				let mut base = base.clone();
				base.push(CalculationResultChip {
					chip_index,
					position,
					rotation
				});
				if chip.rotation != rotation {
					base.correction_cost += chip.get_correction_cost();
				}
				if canvas.get_left_space() < config.min_chip_size {
					on_found(base);
					return
				}
				calculate(&canvas, all_chips, &base, on_found, config)
			},
			&config
		);
	}
}
#[inline(always)]
fn try_put<F: FnMut(Canvas, Vector2<u8>, MatrixRotation)>(canvas: &Canvas, chip: &Chip, on_put: &mut F , config: &Config) {
	let mut matrix_rotation_cache = chip.get_rotation_cache().clone();

	let mut rotation = Cw0;
	for rotation_count in 0..=(
		if config.rotate {
			chip.get_max_rotation()
		} else {
			rotation = chip.rotation;
			0
		}
	) {
		__try_put(
			canvas,
			matrix_rotation_cache.get_mut(&rotation),
			&mut |canvas, pos | on_put.call_mut((canvas, pos, rotation))
		);
		rotation.rotate_cw90();
	}

}
#[inline(always)]
fn __try_put<F: FnMut(Canvas, Vector2<u8>)>(canvas: &Canvas, matrix: &mut Matrix, on_put: &mut F) {
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
				on_put(new_canvas, pos);
				return;
			}
		}
		matrix.shr(1);
	}
	return;
}



#[derive(Clone, Copy, Debug)]
pub struct Config {
	pub min_chip_size: u8,
	pub rotate: bool
}

#[derive(Debug, EnumString)]
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


#[derive(Clone)]
pub struct CalculationResult {
	pub chips: Vec<CalculationResultChip>,
	pub correction_cost: usize,
	pub left_size: u8
}

impl Deref for CalculationResult {
	type Target = Vec<CalculationResultChip>;

	fn deref(&self) -> &Self::Target {
		&self.chips
	}
}

impl DerefMut for CalculationResult {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.chips
	}
}

impl CalculationResult {
	pub fn new(canvas: &Canvas) -> Self {
		Self {
			chips: Default::default(),
			correction_cost: 0,
			left_size: canvas.get_left_space()
		}
	}

	pub fn calculate_stat(&self, all_chips: Vec<Chip>) -> Stat {
		let mut stat = Stat::default();
		for x in &self.chips {
			stat += all_chips.get(x.chip_index).unwrap().get_stat();
		}
		stat
	}

	#[inline(always)]
	pub fn is_used(&self, chip_index: usize) -> bool {
		self.iter().find(| x | x.chip_index == chip_index).is_some()
	}
}

#[derive(Clone)]
pub struct CalculationResultChip {
	pub chip_index: usize,
	pub position: Vector2<u8>,
	pub rotation: MatrixRotation
}