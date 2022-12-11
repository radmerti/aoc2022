use super::item::RucksackItem;

#[derive(Debug)]
pub struct Rucksack {
    compartment_a: String,
    compartment_b: String,
}

impl Rucksack {
    pub fn from_string(string: String) -> Rucksack {
        assert!(
            string.len() % 2 == 0,
            "length of input string must be a multiple of 2"
        );
        let l = string.len() / 2;
        Rucksack {
            compartment_a: string[..l].to_owned(),
            compartment_b: string[l..].to_owned(),
        }
    }

    pub fn types_in_both_compartments(&self) -> Vec<RucksackItem> {
        let mut in_both: Vec<char> = vec![];
        for char_a in self.compartment_a.chars() {
            for char_b in self.compartment_b.chars() {
                if char_a != char_b {
                    continue
                }
                if in_both.contains(&char_a) {
                    continue
                }
                in_both.push(char_a);
            };
        };
        in_both.iter().map(|c| RucksackItem::from_char(*c)).collect()
    }
}