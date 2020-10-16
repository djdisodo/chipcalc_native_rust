use std::ops::{Add, AddAssign};

#[derive(Default, Clone)]
pub struct Stat {
    pub dmg: i32,
    pub brk: i32,
    pub hit: i32,
    pub rld: i32
}

impl Stat {
    pub fn new(dmg: i32, brk: i32, hit: i32, rld: i32) -> Self {
        Self {
            dmg,
            brk,
            hit,
            rld
        }
    }
}

impl Add for Stat {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.dmg += rhs.dmg;
        self.brk += rhs.brk;
        self.hit += rhs.hit;
        self.rld += rhs.rld;
        self
    }
}

impl AddAssign for Stat {
    fn add_assign(&mut self, rhs: Self) {
        self.dmg += rhs.dmg;
        self.brk += rhs.brk;
        self.hit += rhs.hit;
        self.rld += rhs.rld;
    }
}