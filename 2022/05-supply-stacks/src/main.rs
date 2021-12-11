use std::env;
use std::error::Error;
use std::fs;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(Box::from("usage: cargo run -- puzzle-input"));
    }

    let (mut crane, moves) = read_puzzle(&args[1])?;
    let mut crane_9001 = crane.clone();

    crane.start_crane(&moves);

    let part_one = crane.skim()
        .into_iter()
        .filter_map(|top| top)
        .collect::<String>();

    crane_9001.start_crane_9001(&moves);

    let part_two = crane_9001.skim()
        .into_iter()
        .filter_map(|top| top)
        .collect::<String>();

    println!("part one: {:?}", part_one);
    println!("part two: {:?}", part_two);

    Ok(())
}

#[derive(Clone, Debug)]
struct Crane(Vec<Vec<char>>);

impl Crane {
    fn start_crane(&mut self, moves: &Vec<Move>) {
        let Crane(stacks) = self;
        for mov in moves {
            for _ in 0..mov.quantity {
                let grabbed = stacks[mov.from - 1].pop();
                if let Some(label) = grabbed {
                    stacks[mov.to - 1].push(label);
                }
            }
        }
    }

    fn start_crane_9001(&mut self, moves: &Vec<Move>) {
        let Crane(stacks) = self;
        for mov in moves {
            let from = &stacks[mov.from - 1];
            let from_len = from.len();

            // Have to copy instead of using `iter::Extend` because can't have a mutable reference
            // to `mov.to` and a reference to `mov.from` -- yikes.

            if from_len >= mov.quantity {
                let grabbed = &from[from_len - mov.quantity..].iter()
                    .copied()
                    .collect::<Vec<char>>();

                stacks[mov.to - 1].extend(grabbed);

                stacks[mov.from - 1].truncate(from_len - mov.quantity);
            }
        }
    }

    fn skim(&self) -> Vec<Option<char>> {
        let Crane(stacks) = self;
        stacks.iter()
            .map(|stack| stack.last().map(|label| *label))
            .collect()
    }
}

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

fn read_puzzle(puzzle: &str) -> Result<(Crane, Vec<Move>), Box<dyn Error>> {
    // TODO: Splitting on consecutive line feeds isn't portable or potable.
    // Or splitting on newlines in general for that matter.

    let contents = fs::read_to_string(puzzle)?;
    let mut stacks_then_moves = contents.split("\n\n");

    if let (Some(stacks), Some(moves)) = (stacks_then_moves.next(), stacks_then_moves.next()) {
        Ok((read_crane(stacks)?, read_moves(moves.trim())?))
    } else {
        Err(Box::from("expect puzzle to be partitioned by '\\n\\n'"))
    }
}

fn read_crane(text: &str) -> Result<Crane, Box<dyn Error>> {
    let mut lines = text.split('\n').rev();

    let indices = lines.next()
        .ok_or(Box::<dyn Error>::from("expect stack indices"))?
        .trim()
        .split_ascii_whitespace()
        .count();

    let mut stacks = vec![Vec::new(); indices];

    for line in lines {
        let mut chars = line.chars();

        for location in 0..indices {
            let pallet = chars.by_ref().take(4).collect::<String>();
            let label = pallet.chars().nth(1)
                .ok_or(Box::<dyn Error>::from("expect pallet to have label"))?;

            if label != ' ' {
                stacks[location].push(label);
            }
        }
    }

    Ok(Crane(stacks))
}

fn read_moves(text: &str) -> Result<Vec<Move>, Box<dyn Error>> {
    let lines = text.split('\n');

    let mut moves = Vec::new();

    for line in lines {
        let mut mov = line.split_ascii_whitespace();

        let quantity = mov.by_ref().skip(1).next()
            .ok_or(Box::<dyn Error>::from("expect quantity in move"))?
            .parse::<usize>()?;

        let from = mov.by_ref().skip(1).next()
            .ok_or(Box::<dyn Error>::from("expect from in move"))?
            .parse::<usize>()?;

        let to = mov.by_ref().skip(1).next()
            .ok_or(Box::<dyn Error>::from("expect to in move"))?
            .parse::<usize>()?;

        moves.push(Move {
            quantity,
            from,
            to
        });
    }

    Ok(moves)
}
