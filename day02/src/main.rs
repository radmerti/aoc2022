extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod rock_paper_scissors;

use log::{error, info};
use rock_paper_scissors::{game::Game, reader::Reader};

fn main() {
    pretty_env_logger::init();

    let game = Game::new();

    let reader = match Reader::from_path("./input") {
        Ok(reader) => reader,
        Err(error) => {
            error!("error getting reader for input: {}", error);
            return;
        }
    };

    let (mut score_a, mut score_b): (u32, u32) = (0, 0);

    for (round_i, (move_a, outcome)) in reader.enumerate() {
        let move_b = game.move_for(&move_a, &outcome);

        let result = match game.play(&move_a, &move_b) {
            Ok(result) => result,
            Err(error) => {
                error!(
                    "error playing rock-paper-scissors with moves ({}, {}): {}",
                    move_a, move_b, error
                );
                return;
            }
        };

        score_a += result.score_a as u32;
        score_b += result.score_b as u32;

        debug!(
            "round {}: ({}, {}) => {:?}",
            round_i, move_a, move_b, result
        );
    }

    info!("total score A: {}", score_a);
    info!("total score B: {}", score_b);
}
