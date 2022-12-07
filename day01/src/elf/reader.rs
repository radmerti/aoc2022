use super::elf::Elf;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

pub struct ElfCaloriesReader {
    lines: io::Lines<io::BufReader<File>>,
    curr: u32,
}

impl ElfCaloriesReader {
    pub fn from_path<P>(filename: P) -> Result<ElfCaloriesReader, Error>
    where
        P: AsRef<Path>,
    {
        Ok(ElfCaloriesReader {
            lines: read_lines(filename)?,
            curr: 0,
        })
    }
}

impl Iterator for ElfCaloriesReader {
    type Item = Elf;

    fn next(&mut self) -> Option<Self::Item> {

        let mut elf = Elf {
            snacks: vec![],
            line_start: self.curr,
            line_end: self.curr,
        };
        
        while let Ok(text) = self.lines.next()? {
            self.curr += 1;

            // an empty line indicates next elf
            if text == "" {
                break
            };

            // count 0 calories if we cannot parse the number
            let calories = match text.parse() {
                Ok(calories) => calories,
                Err(_) => 0,
            };

            elf.snacks.push(calories);
        }
        
        elf.line_end = self.curr;

        Some(elf)
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
