use std::env;
use std::error::{Error};
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::str::{FromStr};
use std::vec::{Vec};

fn main() -> Result<(), Box<dyn Error>> {
    let elves = read_inventories()?;

    // TODO: I do not understand Rust iterators and closures.
    let mut sums: Vec<u32> = elves.iter()
        .map(|elf| elf.iter().sum())
        .collect();

    sums.sort_unstable();

    let part_one = sums.iter()
        .max()
        .unwrap();

    let part_two = sums.iter()
        .rev()
        .take(3)
        .sum::<u32>();

    println!("part 1: {}", part_one);
    println!("part 2: {}", part_two);

    Ok(())
}

type Inventory = Vec<u32>;

fn read_inventories() -> Result<Vec<Inventory>, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Box::from("usage: solve puzzle-input"));
    }

    let puzzle = &args[1];
    let reader = BufReader::new(File::open(puzzle)?);

    // Behold idiomatic and safe Rust parsing. 🦀

    let mut inventories = Vec::new();
    inventories.push(Vec::new());

    let mut inventory = inventories.last_mut().unwrap();

    for line in reader.lines() {
        // TODO: How does this dereference make &str from &String?
        let line: &str = &line?;

        if line == "" {
            inventories.push(Vec::new());
            inventory = inventories.last_mut().unwrap();
        } else {
            inventory.push(u32::from_str(line)?);
        }
    }

    Ok(inventories)
}
