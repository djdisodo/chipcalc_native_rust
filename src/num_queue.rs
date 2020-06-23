
#[derive(Clone)]
pub struct NumQueue {
	queue: Vec<bool>
}

impl NumQueue {

	pub fn new(size: usize) -> Self {
		Self {
			queue: vec![true; size]
		}
	}

	pub fn peek(&self) -> Option<usize> {
		for i in 0..self.queue.len() {
			if self.queue[i] {
				return Some(i);
			}
		}
		None
	}

	pub fn rm(&mut self, v: usize) {
		self.queue[v] = false
	}

	pub fn push(&mut self, v: usize) {
		self.queue[v] = true
	}
}