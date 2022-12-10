mod elf;

use elf::{reader::ElfCaloriesReader, elf::Elf};
use std::vec::Vec;



fn main() {
    let reader = ElfCaloriesReader::from_path("./input");

    if let Err(error) = reader {
        println!("error getting elf reader: {}", error);
        return
    }

    let mut elfs = reader.unwrap().collect::<Vec<Elf>>();

    for (elf_i, elf) in elfs.iter().enumerate() {
        println!("elf '{}' has {} calories ({:?})", elf_i, elf.total_calories(), elf);
    }

    elfs.sort_unstable_by(|a, b| b.total_calories().cmp(&a.total_calories()));

    println!("Top 3 elfs by total calories.");
    for (elf_i, elf) in elfs[0..3].iter().enumerate() {
        println!("elf '{}' has {} calories ({:?})", elf_i, elf.total_calories(), elf);
    }
    let top_elfs_total_calories: u32 = elfs[0..3].into_iter().map(|e| e.total_calories()).sum();
    println!("total calories of top 3 elfs: {}", top_elfs_total_calories);
}
