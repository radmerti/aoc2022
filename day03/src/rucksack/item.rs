

#[derive(Debug)]
pub struct RucksackItem{
    symbol: char,
}

impl RucksackItem {
    pub fn from_char(c: char) -> RucksackItem {
        assert!(c > 'a' || c < 'z' || c > 'A' || c < 'Z', "a rucksack item must be in a-z, A-Z");
        RucksackItem{symbol: c}
    }

    pub fn priority(&self) -> u8 {
        // "a" to "z" have priorities 1..26
        if self.symbol >= 'a' && self.symbol <= 'z' {
            debug!("symbol: {}, 'a': {}, 'z': {}", self.symbol as u8, 'a' as u8, 'z' as u8);
            return self.symbol as u8 - 'a' as u8 + 1
        };

        // "A" to "Z" have priorities 1..26
        if self.symbol >= 'A' && self.symbol <= 'Z' {
            debug!("symbol: '{}' ({}), 'A': {}, 'Z': {}", self.symbol, self.symbol as u8, 'A' as u8, 'Z' as u8);
            return self.symbol as u8 - 'A' as u8 + 27
        };

        return 0;
    }
}
