use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Rock => write!(f, "Rock"),
            Move::Paper => write!(f, "Paper"),
            Move::Scissors => write!(f, "Scissors"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    WinA,
    WinB,
    Draw,
}

#[derive(Debug)]
pub struct MatchResult {
    pub outcome: Outcome,
    pub score_a: u8,
    pub score_b: u8,
}

#[derive(Debug)]
pub struct Game {}

impl Game {
    pub fn new() -> Game {
        Game {}
    }

    pub fn play(&self, move_a: &Move, move_b: &Move) -> Result<MatchResult, String> {
        let mut score_a = score_move(&move_a);
        let mut score_b = score_move(&move_b);

        let outcome = determine_outcome(&move_a, &move_b)?;

        let (outcome_score_a, outcome_score_b) = score_outcome(&outcome);

        score_a += outcome_score_a;
        score_b += outcome_score_b;

        Ok(MatchResult {
            outcome,
            score_a,
            score_b,
        })
    }
}

fn score_move(m: &Move) -> u8 {
    match m {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn determine_outcome(move_a: &Move, move_b: &Move) -> Result<Outcome, String> {
    match (move_a, move_b) {
        (Move::Rock, Move::Rock)
        | (Move::Paper, Move::Paper)
        | (Move::Scissors, Move::Scissors) => Ok(Outcome::Draw),
        (Move::Rock, Move::Scissors)
        | (Move::Paper, Move::Rock)
        | (Move::Scissors, Move::Paper) => Ok(Outcome::WinA),
        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => Ok(Outcome::WinB),
        _ => Err(format!(
            "combination of moves ({}, {}) is not valid",
            move_a, move_b
        )),
    }
}

fn score_outcome(outcome: &Outcome) -> (u8, u8) {
    match outcome {
        Outcome::WinA => (6, 0),
        Outcome::WinB => (0, 6),
        Outcome::Draw => (3, 3),
    }
}
