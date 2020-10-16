pub struct Stat {
    pub dmg: u32,
    pub brk: u32,
    pub hit: u32,
    pub rld: u32
}

impl Stat {
    pub fn new(dmg: u32, brk: u32, hit: u32, rld: u32) -> Self {
        Self {
            dmg,
            brk,
            hit,
            rld
        }
    }
}