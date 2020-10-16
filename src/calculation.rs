use crate::canvas::Canvas;
use crate::matrix::{MatrixRotationCache, Matrix};
use crate::vector2::Vector2;
use crate::matrix::MatrixRotation::{Cw0, Cw180, Cw90, Cw270};
use crate::matrix::MatrixRotation;
use std::collections::VecDeque;
use crate::chip::Chip;
use crate::shape::Shape;
use strum_macros::EnumString;
use std::ops::{Deref, DerefMut};
use crate::stat::Stat;

pub struct CalculationJob<'a> {
	canvas: Canvas,
	all_chips: &'a Vec<Chip>,
	chips: VecDeque<usize>,
	base: CalculationResult,
	config: Config
}

impl <'a> CalculationJob<'a> {
	pub fn new(
		canvas: Canvas,
		all_chips: &'a Vec<Chip>,
		chips: VecDeque<usize>,
		base: CalculationResult,
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

	pub fn generate_jobs(self) -> GenerateJob<'a> {
		GenerateJob::new(self)
	}

	pub fn calculate(&self) -> Vec<CalculationResult> {
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
	job: CalculationJob<'a>,
	cache: VecDeque<CalculationJob<'a>>
}

impl <'a> GenerateJob<'a> {
	pub fn new(job: CalculationJob<'a>) -> Self {
		Self {
			job,
			cache: VecDeque::with_capacity(4)
		}
	}
}

impl <'a> Iterator for GenerateJob<'a> {
	type Item = CalculationJob<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		while self.cache.is_empty() {
			let chip_index = self.job.chips.pop_front();
			if chip_index.is_none() {
				return None;
			}
			let chip = self.job.all_chips.get(chip_index.unwrap()).unwrap();
			let mut chips = self.job.chips.clone();
			let rotate = self.job.config.rotate;
			let cache = &mut self.cache;
			let job = &self.job;
			try_put(
				&self.job.canvas,
				chip,
				| canvas, pos, rotation | {
					let mut base = job.base.clone();
					base.push(CalculationResultChip {
						chip_index: chip_index.unwrap(),
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
							job.chips.clone(),
							base,
							job.config
						),
					);
					true
				},
				&self.job.config
			)
		}
		self.cache.pop_front()
	}
}

fn calculate<'a>(
	canvas: &Canvas,
	all_chips: &'a Vec<Chip>,
	chips: &VecDeque<usize>,
	base: &CalculationResult,
	config: &Config
) -> Option<Vec<CalculationResult>> {
	let mut result = Vec::new();
	let mut chips = chips.clone();
	let mut chip_index = chips.pop_front();

	while chip_index.is_some() {
		let chip = all_chips.get(chip_index.unwrap()).unwrap();
		try_put(
			canvas,
			chip,
			| canvas, position, rotation | {
				let mut base = base.clone();
				base.push(CalculationResultChip {
					chip_index: chip_index.unwrap(),
					position,
					rotation
				});
				if chip.rotation != rotation {
					base.correction_cost += chip.get_correction_cost();
				}
				let r = calculate(&canvas, all_chips, &chips, &base, config);
				if r.is_some() {
					result.append(&mut r.unwrap());
				}
				canvas.get_left_space() != 0
			},
			&config
		);
		chip_index = chips.pop_front();
	}
	if result.is_empty() {
		result.push(base.clone());
		return Some(result);
	}
	Some(result)
}

fn try_put(canvas: &Canvas, chip: &Chip, mut on_put: impl FnMut(Canvas, Vector2<u8>, MatrixRotation) -> bool, config: &Config) {
	let mut matrix_rotation_cache = chip.get_rotation_cache().clone();
	let mut rot = *chip.get_rotation();

	__try_put(
		canvas,
		matrix_rotation_cache.get_mut(&rot),
		|canvas, pos | on_put.call_mut((canvas, pos, rot)),
		config.allow_space
	);

	if config.rotate {
		for _ in 0..chip.get_max_rotation() {
			rot.rotate_cw90();
			__try_put(
				canvas,
				matrix_rotation_cache.get_mut(&rot),
				|canvas, pos | on_put.call_mut((canvas, pos, rot)),
				config.allow_space
			);
		}
	}

}

fn __try_put(canvas: &Canvas, matrix: &mut Matrix, mut on_put: impl FnMut(Canvas, Vector2<u8>) -> bool, allow_space: bool) {
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
				if !allow_space {
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
	pub rotate: bool,
	pub allow_space: bool
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