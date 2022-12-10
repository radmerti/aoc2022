use super::game::{Move, Outcome};
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
    type Item = (Move, Outcome);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?;

        let text = match line {
            Ok(text) => text,
            Err(error) => {
                println!("error decoding line: {}", error);
                return None;
            }
        };

        let moves: Vec<&str> = text.split(" ").collect();

        let move_a = match moves[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => return None,
        };

        let move_b = match moves[1] {
            "X" => Outcome::WinA,
            "Y" => Outcome::Draw,
            "Z" => Outcome::WinB,
            _ => return None,
        };

        Some((move_a, move_b))
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
