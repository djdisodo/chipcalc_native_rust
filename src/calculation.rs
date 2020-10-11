use crate::canvas::Canvas;
use crate::matrix::{Matrix, MatrixRotationCache};
use crate::vector2::Vector2;
use crate::calculation::ChipRotation::{Cw0, Cw180, Cw90, Cw270};
use std::collections::VecDeque;

pub struct CalculationJob<'a> {
	canvas: Canvas,
	all_chips: &'a Vec<MatrixRotationCache>,
	chips: VecDeque<usize>,
	base: Vec<(usize, Vector2<u8>, ChipRotation)>,
	config: Config
}

impl <'a> CalculationJob<'a> {
	pub fn new(
		canvas: Canvas,
		all_chips: &'a Vec<MatrixRotationCache>,
		chips: VecDeque<usize>,
		base: Vec<(usize, Vector2<u8>, ChipRotation)>,
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

	pub fn calculate(&self, max_left_space: u8) -> Vec<Vec<(usize, Vector2<u8>, ChipRotation)>> {
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
	all_chips: &'a Vec<MatrixRotationCache>,
	chips: &VecDeque<usize>,
	base: &Vec<(usize, Vector2<u8>, ChipRotation)>,
	config: &Config
) -> Option<Vec<Vec<(usize, Vector2<u8>, ChipRotation)>>> {
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

fn try_put(canvas: &Canvas, chip: &MatrixRotationCache, mut on_put: impl FnMut(Canvas, Vector2<u8>, ChipRotation) -> bool, rotate: bool) {
	let mut chip = chip.clone();
	try_put_for_rotation(canvas, &mut chip, Cw0, |canvas, pos | on_put.call_mut((canvas, pos, Cw0)));
	if rotate {
		try_put_for_rotation(canvas, &mut chip, Cw90, |canvas, pos | on_put.call_mut((canvas, pos, Cw90)));
		try_put_for_rotation(canvas, &mut chip, Cw180, |canvas, pos | on_put.call_mut((canvas, pos, Cw180)));
		try_put_for_rotation(canvas, &mut chip, Cw270, |canvas, pos | on_put.call_mut((canvas, pos, Cw270)));
	}

}

fn try_put_for_rotation(canvas: &Canvas, chip: &mut MatrixRotationCache, rotation: ChipRotation, mut on_put: impl FnMut(Canvas, Vector2<u8>) -> bool) {
	let chip = match rotation {
		Cw0 => &mut chip.cw0,
		Cw90 => &mut chip.cw90,
		Cw180 => &mut chip.cw180,
		Cw270 => &mut chip.cw270
	};
	for x in 0..canvas.size.x {
		if chip.size.x + x > canvas.size.x {
			break;
		}
		for y in 0..canvas.size.y {
			if chip.size.y + y > canvas.size.y {
				break;
			}
			let mut new_canvas = canvas.clone();
			let mut fit = true;
			for i in 0..chip.size.y as usize {
				if new_canvas.raw_map[i + y as usize] & chip.raw_map[i] == 0b0 {
					new_canvas.raw_map[i + y as usize] |= chip.raw_map[i];
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
		chip.shr(1);
	}
	return;
}


#[derive(Clone, Copy, Debug)]
pub enum ChipRotation {
	Cw0,
	Cw90,
	Cw180,
	Cw270
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
	pub max_left_space: u8,
	pub rotate: bool
}

pub fn bytemap_to_bitmap(input: &[[bool; 8]]) -> Vec<Vec<u8>> {
	unimplemented!()
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
		let mut map: [[u8; 8]; 8] = match self {
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