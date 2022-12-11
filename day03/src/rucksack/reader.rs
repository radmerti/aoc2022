use super::rucksack::Rucksack;
use std::io::{BufRead, Error};
use std::{fs::File, io, path::Path};

pub struct Reader {
    lines: io::Lines<io::BufReader<File>>,
}

impl Reader {
    pub fn from_path<P>(filename: P) -> Result<Reader, Error>
    where
        P: AsRef<Path>,
    {
        Ok(Reader {
            lines: read_lines(filename)?,
        })
    }
}

impl Iterator for Reader {
    type Item = Rucksack;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;

        let text = match line {
            Ok(text) => text,
            Err(error) => {
                println!("error decoding line: {}", error);
                return None;
            }
        };

        Some(Rucksack::from_string(text))
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
