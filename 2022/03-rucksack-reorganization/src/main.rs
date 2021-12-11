/*
 * - Each rucksack has 2 large compartments.
 * - All items of a given type are meant to go in exactly 1 of the 2 compartments.
 * - An elf has failed to do so for exactly 1 item type per rucksack.
 * - Every item type is identified by a single lowercase or uppercase letter.
 *
 *              2nd compartment
 *              v
 * vJrwpWtwJgWr hcsFMMfFFhFp
 * ^                       ^
 * 1st compartment         item type common to both rucksacks
 *
 */

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    let rucksacks = read_rucksacks()?;

    let part_one = rucksacks.iter()
        .map(Rucksack::repack)
        .map(|common| {
            let common = common.into_iter().collect::<Vec<char>>();
            assert!(common.len() == 1, "expect exactly one item between compartments");
            common[0]
        })
        .filter_map(Rucksack::priority)
        .map(|priority| priority as u32)
        .sum::<u32>();

    let part_two = rucksacks.chunks_exact(3)
        .map(|group| Rucksack::badge_candidates(group.iter()))
        .map(|badges| {
            let badges = badges.into_iter().collect::<Vec<char>>();
            assert!(badges.len() == 1, "expect exactly one badge between a group of elves");
            badges[0]
        })
        .filter_map(Rucksack::priority)
        .map(|priority| priority as u32)
        .sum::<u32>();

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);

    Ok(())
}

type Compartment = HashSet<char>;

#[derive(Debug)]
struct Rucksack {
    top: Compartment,
    bottom: Compartment,
}

impl Rucksack {
    fn from_compartments(items: &str, more_items: &str) -> Rucksack {
        let mut rucksack = Rucksack {
            top: HashSet::new(),
            bottom: HashSet::new()
        };

        Rucksack::fill(items, &mut rucksack.top);
        Rucksack::fill(more_items, &mut rucksack.bottom);

        rucksack
    }

    fn fill(items: &str, compartment: &mut Compartment) {
        for item in items.chars() {
            compartment.insert(item);
        }
    }

    fn repack(&self) -> HashSet<char> {
        self.top.intersection(&self.bottom).copied().collect()
    }

    fn badge_candidates<'a>(sacks: impl Iterator<Item=&'a Self>) -> HashSet<char> {
        let contents = sacks.map(|sack| sack.top.union(&sack.bottom)
            .copied()
            .collect::<HashSet<char>>()
        );

        contents.reduce(
            |in_all, sack| in_all.intersection(&sack)
                .copied()
                .collect()
        ).unwrap_or_default()
    }

    fn priority(item: char) -> Option<u8> {
        if item.is_ascii() {
            let ascii_value = item as u8;

            if item.is_uppercase() {
                return Some(27 + (ascii_value - b'A'));
            } else {
                return Some(1 + (ascii_value - b'a'));
            };
        }

        None
    }
}

fn read_rucksacks() -> Result<Vec<Rucksack>, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Box::from("usage: solve puzzle-input"));
    }

    let puzzle = &args[1];
    let reader = BufReader::new(File::open(puzzle)?);

    let mut rucksacks = Vec::new();

    for line in reader.lines() {
        let items = line?;
        let compartment_size = items.len() / 2;

        rucksacks.push(Rucksack::from_compartments(
            &items[..compartment_size],
            &items[compartment_size..]
        ));
    }

    Ok(rucksacks)
}
