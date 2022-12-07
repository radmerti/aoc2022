#[derive(Debug)]
pub struct Elf {
    pub snacks: Vec<u32>,
    pub line_start: u32,
    pub line_end: u32,
}

impl Elf {
    pub fn total_calories(&self) -> u32 {
        self.snacks.iter().sum()
    }
}
