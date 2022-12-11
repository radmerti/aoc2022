extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use rucksack::reader::Reader;

mod rucksack;

fn main() {

    pretty_env_logger::init();

    let reader = match Reader::from_path("./input") {
        Ok(reader) => reader,
        Err(error) => {
            error!("error getting reader for rucksacks: {}", error);
            return
        }
    };

    let mut total_priority_in_both: u64 = 0;
    for (rucksack_i, rucksack) in reader.enumerate() {
        let in_both =  rucksack.types_in_both_compartments();
        let priority: u8 = in_both.iter().map(|i| i.priority()).sum();
        total_priority_in_both += priority as u64;
        info!("in both: {:?}, priority: {}", in_both, priority);
        debug!("rucksack {}: {:?}", rucksack_i, rucksack);
    }
    info!("total priority in both: {}", total_priority_in_both);
}
