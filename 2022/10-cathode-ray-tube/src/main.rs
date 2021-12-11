use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
enum Stmt {
    Addx(i32),
    Noop
}

#[derive(Debug, PartialEq)]
struct Ins {
    stmt: Stmt,
    cycles: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(Box::from("usage: cargo run -- puzzle-input"));
    }

    let puzzle = fs::read_to_string(&args[1])?;
    let instructions = read_puzzle(&puzzle)?;

    let desired = (20..=220).step_by(40).collect::<Vec<i32>>();
    let values = read_register(&instructions, &desired);

    let part_one =
        desired.iter()
        .zip(values.iter())
        .map(|(&cycle, &value)| cycle * value)
        .sum::<i32>();

    println!("part one: {:?}", part_one);

    println!("part two");
    render(&instructions);

    Ok(())
}

fn read_register(instructions: &[Ins], cycles: &[i32]) -> Vec<i32> {
    // TODO: Slow, use a tree and find the largest key less than or equal to the desired cycle.
    // The floor of the desired cycle.

    let measures = inspect_register(instructions);
    let mut values = Vec::new();

    for &cycle in cycles {
        let register = measures.iter()
            .rev()
            // Will fail only if cycle is non-positive. If the cycle is invalid, ignore it.
            .find(|&&(rising_edge, _)| rising_edge <= cycle)
            .map(|measure| measure.1);

        if let Some(value) = register {
            values.push(value);
        }
    }

    values
}

fn inspect_register(instructions: &[Ins]) -> Vec<(i32, i32)> {
    // For a pair `(n, x)`, at the start of cycle `n` the value of the register is `x`.
    let mut measurements = Vec::new();

    let mut register = 1;
    let mut cycle = 1;

    measurements.push((cycle, register));

    for instruction in instructions {
        cycle += instruction.cycles;

        if let &Ins { stmt: Stmt::Addx(increment), .. } = instruction {
            if increment != 0 {
                register += increment;
                measurements.push((cycle, register));
            }
        }
    }

    measurements
}

fn render(instructions: &[Ins]) {
    let crt_height = 6;
    let crt_width = 40;
    let crt_pixels = crt_height * crt_width;

    let measures = inspect_register(&instructions);
    let mut lit = HashSet::new();

    for i in 0..measures.len() {
        let sprite_center = measures[i].1;

        let (cycle_start, cycle_end) = if i < measures.len() - 1 {
            (measures[i].0, measures[i + 1].0 - 1)
        } else {
            (measures[i].0, crt_pixels)
        };

        let (draw_start, draw_end) = (
            (cycle_start - 1) % crt_width,
            (cycle_end - 1) % crt_width
        );

        for sprite in [sprite_center - 1, sprite_center, sprite_center + 1] {
            if draw_start <= sprite && sprite <= draw_end {
                lit.insert((cycle_start / crt_width, sprite));
            }
        }
    }

    for row in 0..crt_height {
        for column in 0..crt_width {
            if lit.contains(&(row, column)) {
                print!("🟨");
            } else {
                print!("⬛️");
            }
        }
        println!();
    }
}

fn read_puzzle(puzzle: &str) -> Result<Vec<Ins>, Box<dyn Error>> {
    let mut instructions = Vec::new();

    for line in puzzle.lines() {
        let mut pieces = line.split_ascii_whitespace();
        let instruction = pieces.next().ok_or("expect `addx` or `noop`")?;
        instructions.push(match instruction {
            "addx" => {
                let increment = pieces.next()
                    .ok_or("expect `addx` to have an argument")?
                    .parse::<i32>()?;
                Ins { stmt: Stmt::Addx(increment), cycles: 2 }
            },
            "noop" =>
                Ins { stmt: Stmt::Noop, cycles: 1 },
            oops =>
                Err(Box::<dyn Error>::from(format!("unexpected instruction `{}`", oops)))?
        });
    }

    Ok(instructions)
}

#[test]
fn tiny_sample() -> Result<(), Box<dyn Error>> {
    let sample = r"noop
        addx 3
        addx -5";

    let ins = read_puzzle(sample)?;
    let measures = inspect_register(&ins);
    assert_eq!(measures, vec![(1, 1), (4, 4), (6, -1)]);

    Ok(())
}

#[test]
fn tailing_noop() -> Result<(), Box<dyn Error>> {
    let sample = r"noop
        addx 3
        addx -5
        noop";

    let ins = read_puzzle(sample)?;
    let measures = inspect_register(&ins);
    assert_eq!(measures, vec![(1, 1), (4, 4), (6, -1)]);

    Ok(())
}
