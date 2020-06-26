use crate::canvas::Canvas;
use crate::chip::{Chip, ChipRotationCache};
use crate::vector2::Vector2;
use crate::calculation::ChipRotation::{Cw0, Cw180, Cw90, Cw270};
use std::collections::VecDeque;

pub struct CalculationJob<'a> {
	canvas: Canvas,
	chips: VecDeque<&'a ChipRotationCache>,
	base: Vec<(&'a ChipRotationCache, Vector2<u8>, ChipRotation)>
}

impl <'a> CalculationJob<'a> {
	pub fn new(canvas: Canvas, chips: VecDeque<&'a ChipRotationCache>, base: Vec<(&'a ChipRotationCache, Vector2<u8>, ChipRotation)>) -> Self {
		Self {
			canvas,
			chips,
			base
		}
	}

	pub fn generate_jobs(&self) -> GenerateJob {
		GenerateJob::new(self)
	}

	pub fn calculate(&self, max_left_space: u8) -> Vec<Vec<(&'a ChipRotationCache, Vector2<u8>, ChipRotation)>> {
		let result = calculate(&self.canvas, &self.chips, &self.base, max_left_space);
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
	chips: VecDeque<&'a ChipRotationCache>,
	chips_base: VecDeque<&'a ChipRotationCache>,
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
			let pos = chips.iter().position(
				| x | *x as *const ChipRotationCache == chip.unwrap() as *const ChipRotationCache
			).unwrap();
			chips.remove(pos);
			try_put(
				&self.job.canvas,
				chip.unwrap(),
				| canvas, pos, rotation | {
					let mut base = self.job.base.clone();
					base.push((chip.unwrap(), pos, rotation));
					self.cache.push_back(
					CalculationJob::new(
						self.job.canvas.clone(),
						chips.clone(),
							base
						)
					);
					true
				}
			)
		}
		self.cache.pop_front()
	}
}

fn calculate<'a>(canvas: &Canvas, chips: &VecDeque<&'a ChipRotationCache>, base: &Vec<(&'a ChipRotationCache, Vector2<u8>, ChipRotation)>, max_left_space: u8) -> Option<Vec<Vec<(&'a ChipRotationCache, Vector2<u8>, ChipRotation)>>> {
	let mut result = Vec::new();
	if canvas.get_left_space() <= max_left_space {
		return None;
	}
	let mut chips = chips.clone();
	let mut chip = chips.pop_front();
	while chip.is_some() {
		try_put(
			canvas,
			chip.unwrap(),
			| canvas, position, rotation | {
				let mut base = base.clone();
				base.push((chip.unwrap(), position.clone(), rotation.clone()));
				let r = calculate(&canvas, &chips, &base, max_left_space);
				if r.is_some() {
					result.append(&mut r.unwrap());
				}
				canvas.get_left_space() != 0
			}
		);
		chip = chips.pop_front();
	}
	if result.is_empty() {
		result.push(base.clone());
		return Some(result);
	}
	Some(result)
}

fn try_put(canvas: &Canvas, chip: &ChipRotationCache, mut on_put: impl FnMut(Canvas, Vector2<u8>, ChipRotation) -> bool) {
	let mut chip = chip.clone();
	try_put_for_rotation(canvas, &mut chip, Cw0, |canvas, pos | on_put.call_mut((canvas, pos, Cw0)));
	try_put_for_rotation(canvas, &mut chip, Cw90, |canvas, pos | on_put.call_mut((canvas, pos, Cw90)));
	try_put_for_rotation(canvas, &mut chip, Cw180, |canvas, pos | on_put.call_mut((canvas, pos, Cw180)));
	try_put_for_rotation(canvas, &mut chip, Cw270, |canvas, pos | on_put.call_mut((canvas, pos, Cw270)));
}

fn try_put_for_rotation(canvas: &Canvas, chip: &mut ChipRotationCache, rotation: ChipRotation, mut on_put: impl FnMut(Canvas, Vector2<u8>) -> bool) {
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