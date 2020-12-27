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
	next_chip: usize,
	base: CalculationResult,
	config: Config
}

impl <'a> CalculationJob<'a> {
	pub fn new(
		canvas: Canvas,
		all_chips: &'a Vec<Chip>,
		next_chip: usize,
		base: CalculationResult,
		config: Config
	) -> Self {
		Self {
			canvas,
			all_chips,
			next_chip,
			base,
			config
		}
	}

	pub fn generate_jobs(self) -> GenerateJob<'a> {
		GenerateJob::new(self)
	}

	pub fn calculate(&self) -> Option<Vec<CalculationResult>> {
		calculate(
			&self.canvas,
			self.all_chips,
			self.next_chip,
			&self.base,
			&self.config
		)
	}
}

pub struct GenerateJob<'a> {
	job: CalculationJob<'a>,
	x_range: u8,
	x: u8,
	y_range: u8,
	y: u8,
	matrixes: Vec<MatrixRotationCache>,
	cache: VecDeque<CalculationJob<'a>>
}

impl <'a> GenerateJob<'a> {
	pub fn new(job: CalculationJob<'a>) -> Self {
		let x_range = job.canvas.size.x;
		let y_range = job.canvas.size.y;
		let chip_range = job.next_chip..job.all_chips.len();
		let matrixes: Vec<MatrixRotationCache> = job.all_chips.iter().map(| x | x.shape.get_rotation_cache().clone()).collect();
		Self {
			job,
			x_range,
			x: 0,
			y_range,
			y: 0,
			matrixes,
			cache: VecDeque::with_capacity(4)
		}
	}
}

impl <'a> Iterator for GenerateJob<'a> {
	type Item = CalculationJob<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.cache.is_empty() {
			if self.x < self.x_range {
				if self.y < self.y_range {
					if self.job.next_chip < self.job.all_chips.len() {
						let chip_index = self.job.next_chip;

						let chip = &self.job.all_chips[chip_index];
						let mut matrix_rotation_cache = &mut self.matrixes[chip_index];
						let mut rotation = chip.rotation.clone();
						for rotation_count in 0..(
							if self.job.config.rotate {
								chip.get_max_rotation() + 1
							} else {
								1
							}
						) {
							let matrix = matrix_rotation_cache.get_mut(&rotation);
							if
							matrix.x_size + self.x > self.job.canvas.size.x ||
								(matrix.raw_map.len() as u8) + self.y > self.job.canvas.size.y
							{
								break;
							}
							if let Some(new_canvas) = fit_chip(self.job.canvas.clone(), matrix, self.y as usize) {
								let mut calculation_result = self.job.base.clone();
								calculation_result.push(CalculationResultChip {
									chip_index,
									position: Vector2::new(self.x, self.y),
									rotation
								});
								if rotation_count != 0 {
									calculation_result.correction_cost += chip.get_correction_cost();
								}
								self.cache.push_back(CalculationJob::new(
									new_canvas,
									self.job.all_chips,
									chip_index + 1,
									calculation_result,
									self.job.config
								));
							}
							rotation.rotate_cw90();
						}
						self.job.next_chip += 1;
						continue;
					} else {
						self.job.next_chip = 0;
						self.y += 1;
					}
				} else {
					self.y = 0;
					self.x += 1;
					for chip_index in self.job.next_chip..self.job.all_chips.len() {
						let chip = &self.job.all_chips[chip_index];
						let matrix_rotation_cache = &mut self.matrixes[chip_index];
						let mut rotation = chip.rotation.clone();
						for rotation_count in 0..(
							if self.job.config.rotate {
								chip.get_max_rotation() + 1
							} else {
								1
							}
						) {
							matrix_rotation_cache.get_mut(&rotation).shr(1);
						}
					}
				}
			} else {
				return None;
			}
		}
		self.cache.pop_front()
	}
}

fn calculate<'a>(
	canvas: &Canvas,
	all_chips: &'a Vec<Chip>,
	mut next_chip: usize,
	base: &CalculationResult,
	config: &Config
) -> Option<Vec<CalculationResult>> {
	let mut return_calculation_results = Vec::new();

	let mut matrixes: Vec<MatrixRotationCache> = all_chips.iter().map(| x | x.shape.get_rotation_cache().clone()).collect();
	for x in 0..canvas.size.x {
		for y in 0..canvas.size.y {
			for chip_index in next_chip..all_chips.len() {
				let chip = &all_chips[chip_index];
				let matrix_rotation_cache = &matrixes[chip_index];
				let mut rotation = chip.rotation.clone();
				for rotation_count in 0..(
					if config.rotate {
						chip.get_max_rotation() + 1
					} else {
						1
					}
				) {
					let matrix = matrix_rotation_cache.get(&rotation);
					if
						matrix.x_size + x > canvas.size.x ||
						(matrix.raw_map.len() as u8) + y > canvas.size.y
					{
						break;
					}
					if let Some(new_canvas) = fit_chip(canvas.clone(), matrix, y as usize) {
						let mut calculation_result = base.clone();
						calculation_result.push(CalculationResultChip {
							chip_index,
							position: Vector2::new(x, y),
							rotation
						});
						if rotation_count != 0 {
							calculation_result.correction_cost += chip.get_correction_cost();
						}
						if new_canvas.get_left_space() < config.min_chip_size {
							return_calculation_results.push(calculation_result);
							break;
						}
						if
							let Some(mut calculation_results) =
								calculate(&new_canvas, all_chips, chip_index + 1,
								            &calculation_result,  config
								)
						{
							return_calculation_results.append(&mut calculation_results);
						}
					}
					rotation.rotate_cw90();
				}
			}
		}
		for chip_index in next_chip..all_chips.len() {
			let chip = &all_chips[chip_index];
			let matrix_rotation_cache = &mut matrixes[chip_index];
			let mut rotation = chip.rotation.clone();
			for rotation_count in 0..(
				if config.rotate {
					chip.get_max_rotation() + 1
				} else {
					1
				}
			) {
				matrix_rotation_cache.get_mut(&rotation).shr(1);
			}
		}
	}

	if return_calculation_results.is_empty() {
		None
	} else {
		Some(return_calculation_results)
	}
}

#[inline(always)]
fn fit_chip(mut canvas: Canvas, matrix: &Matrix, y: usize) -> Option<Canvas> {
	let mut fit = true;
	for i in 0..matrix.raw_map.len() {
		if canvas.raw_map[i + y as usize] & matrix.raw_map[i] == 0b0 {
			canvas.raw_map[i + y as usize] |= matrix.raw_map[i];
		} else {
			fit = false;
			break;
		}
	}
	if fit {
		Some(canvas)
	} else {
		None
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
	pub min_chip_size: u8,
	pub rotate: bool,
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


#[derive(Default, Clone)]
pub struct CalculationResult {
	pub chips: Vec<CalculationResultChip>,
	pub correction_cost: usize
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
	pub fn calculate_stat(&self, all_chips: Vec<Chip>) -> Stat {
		let mut stat = Stat::default();
		for x in &self.chips {
			stat += all_chips.get(x.chip_index).unwrap().get_stat();
		}
		stat
	}
}

#[derive(Clone)]
pub struct CalculationResultChip {
	pub chip_index: usize,
	pub position: Vector2<u8>,
	pub rotation: MatrixRotation
}