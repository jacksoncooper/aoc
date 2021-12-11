use std::env;
use std::error::{Error};
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::vec::{Vec};

fn main() -> Result<(), Box<dyn Error>> {
    let (rounds, partial_rounds) = read_strategy()?;

    let part_one: u32 = rounds.iter()
        .map(Round::judge)
        .sum();

    let part_two: u32 = partial_rounds.iter()
        .map(Round::from_partial)
        .map(|ref round| Round::judge(round))
        .sum();

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Debug)]
struct Round {
    elf: Shape,
    you: Shape,
}

struct PartialRound {
    elf: Shape,
    you: Outcome,
}

fn read_strategy() -> Result<(Vec<Round>, Vec<PartialRound>), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Box::from("usage: solve puzzle-input"));
    }

    let puzzle = &args[1];
    let reader = BufReader::new(File::open(puzzle)?);

    let mut rounds = Vec::new();
    let mut partial_rounds = Vec::new();

    for line in reader.lines() {
        // TODO: How does this dereference make &str from &String?
        let line: &str = &line?;
        let mut round = line.split_whitespace();

        let elf = round.next().expect("expect two labels in round");
        let you = round.next().expect("expect two labels in round");

        let strategies = (Round::from_strategy(elf, you), PartialRound::from_strategy(elf, you));

        if let (Some(round), Some(partial_round)) = strategies {
            rounds.push(round);
            partial_rounds.push(partial_round);
        } else {
            let malformed_round = format!("unexpected round ('{}', '{}')", elf, you);
            return Err(Box::from(malformed_round));
        }
    }

    Ok((rounds, partial_rounds))
}

impl Shape {
    fn from_strategy(shape_label: &str, labels: [&str; 3]) -> Option<Self> {
        Some([
            (labels[0], Shape::Rock),
            (labels[1], Shape::Paper),
            (labels[2], Shape::Scissors)
        ].iter()
        .find(|(label, _)| *label == shape_label)?.1)
    }
}

impl PartialRound {
    fn from_strategy(elf_label: &str, you_label: &str) -> Option<Self> {
        let elf = Shape::from_strategy(elf_label, ["A", "B", "C"])?;
        let you = [
            ("X", Outcome::Lose),
            ("Y", Outcome::Draw),
            ("Z", Outcome::Win),
        ].iter()
        .find(|(label, _)| *label == you_label)?.1;

        Some(PartialRound{ elf, you })
    }
}

impl Round {
    fn from_strategy(elf_label: &str, you_label: &str) -> Option<Self> {
        let elf = Shape::from_strategy(elf_label, ["A", "B", "C"])?;
        let you = Shape::from_strategy(you_label, ["X", "Y", "Z"])?;
        Some(Round{ elf, you })
    }

    fn from_partial(partial: &PartialRound) -> Round {
        let you = match partial.you {
            Outcome::Lose => {
                match partial.elf {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                }
            },
            Outcome::Draw => partial.elf,
            Outcome::Win => {
                match partial.elf {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                }
            },
        };

        Round {
            elf: partial.elf,
            you
        }
    }

    fn judge(round: &Self) -> u32 {
        let daring = match round.you {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let performance = match round {
            Round { you: Shape::Rock, elf: Shape::Paper} => 0,
            Round { you: Shape::Paper, elf: Shape::Scissors} => 0,
            Round { you: Shape::Scissors, elf: Shape::Rock} => 0,
            Round { you, elf } if you == elf => 3,
            _ => 6,
        };

        daring + performance
    }
}
