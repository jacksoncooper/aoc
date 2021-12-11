/*
 * - Every section of the camp is labeled with a number.
 * - Elves are assigned a range of sections to clean.
 */

use std::env;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    let reader = read()?;
    let pairs = read_pairs(reader)?;

    let part_one = pairs.iter()
        .map(|(left, right)| left.contains(right) || right.contains(left))
        .filter(|&contains| contains)
        .count();

    let part_two = pairs.iter()
        .map(|(left, right)| left.overlaps(right))
        .filter(|&contains| contains)
        .count();

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);

    Ok(())
}

#[derive(Debug)]
struct Interval(u32, u32);

impl FromStr for Interval {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut endpoints = value.split('-');

        if let (Some(lower), Some(upper)) = (endpoints.next(), endpoints.next()) {
            let (lower, upper) = (lower.parse()?, upper.parse()?);

            if lower > upper {
                return Err(Box::from("expect lower endpoint to be less than upper"));
            }

            Ok(Interval(lower, upper))
        } else {
            Err(Box::from("expect exactly one elision '-' in interval"))
        }
    }
}

impl Interval {
    fn contains_point(&self, point: u32) -> bool {
        self.0 <= point && point <= self.1
    }

    fn contains(&self, other: &Self) -> bool {
        return self.contains_point(other.0) && self.contains_point(other.1)
    }

    fn overlaps(&self, other: &Self) -> bool {
        return self.contains_point(other.0) || self.contains_point(other.1) || other.contains(self)
    }
}

fn read() -> Result<impl BufRead, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Box::from("usage: solve puzzle-input"));
    }

    let puzzle = &args[1];
    let reader = BufReader::new(File::open(puzzle)?);

    Ok(reader)
}

fn read_pairs(reader: impl BufRead) -> Result<Vec<(Interval, Interval)>, Box<dyn Error>> {
    let mut pairs = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let mut intervals = line.split(',');

        if let (Some(left), Some(right)) = (intervals.next(), intervals.next()) {
            pairs.push((left.parse()?, right.parse()?))
        } else {
            return Err(Box::from("expect exactly two intervals in pair"));
        }
    }

    Ok(pairs)
}
