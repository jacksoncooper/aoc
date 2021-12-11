use std::collections::{BinaryHeap, VecDeque};
use std::env;
use std::error::Error;
use std::fs;

type Worry = usize;
type Label = usize;

struct Throw {
    divisor: usize,
    test: Box<dyn Fn(Worry) -> bool>,
    success: Label,
    failure: Label,
}

struct Monkey {
    _label: Label,
    items: VecDeque<Worry>,
    inspect: Box<dyn Fn(Worry) -> Worry>,
    inspections: usize,
    throw: Throw,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();
    let puzzle = fs::read_to_string(&args[1])?;

    if args.len() < 2 {
        return Err(Box::from("usage: cargo run -- puzzle-input"));
    }

    let (largest, second_largest) = simulate(&puzzle, 20, false);
    println!("part one: {} * {} = {}", largest, second_largest, largest * second_largest);

    let (largest, second_largest) = simulate(&puzzle, 10000, true);
    println!("part two: {} * {} = {}", largest, second_largest, largest * second_largest);

    Ok(())
}

fn simulate(puzzle: &str, rounds: usize, ludicrous_speed: bool) -> (usize, usize) {
    // TODO: Can't clone the monkeys because trait objects are not `Clone`. Have to read the input
    // twice.

    let mut monkeys = read_monkeys(&puzzle);

    for _ in 1..=rounds {
        round(&mut monkeys, ludicrous_speed);
    }

    let mut inspections = monkeys.iter()
        .map(|monkey| monkey.inspections)
        .collect::<BinaryHeap<usize>>();

    let largest = inspections.pop().unwrap();
    let second_largest = inspections.pop().unwrap();

    (largest, second_largest)
}

fn round(monkeys: &mut Vec<Monkey>, ludicrous_speed: bool) {
    let item_modulus = monkeys.iter()
        .map(|monkey| monkey.throw.divisor)
        .product::<usize>();

    for i in 0..monkeys.len() {
        while monkeys[i].items.len() > 0 {
            let monkey = &mut monkeys[i];

            let item = monkey.items.pop_front().unwrap();

            let mut new_item = (monkey.inspect)(item);

            if !ludicrous_speed {
                new_item = new_item / 3;
            }

            new_item = new_item % item_modulus;

            monkey.inspections += 1;

            let (success, failure) = (monkey.throw.success, monkey.throw.failure);
            if (monkey.throw.test)(new_item) {
                monkeys[success].items.push_back(new_item);
            } else {
                monkeys[failure].items.push_back(new_item);
            }
        }
    }
}

fn read_monkeys(notes: &str) -> Vec<Monkey> {
    notes.split("\n\n")
        .flat_map(read_monkey) // Danger! Skips garbage.
        .collect::<Vec<Monkey>>()
}

fn read_monkey(notes: &str) -> Option<Monkey> {
    let mut lines = notes.lines()
        .map(|line| line.trim());

    let label = lines.next()?
        .strip_prefix("Monkey ")?
        .strip_suffix(":")?
        .parse::<usize>().ok()?;

    let items = lines.next()?
        .strip_prefix("Starting items: ")?
        .split(", ")
        .flat_map(|item| item.parse::<usize>().ok()) // Danger! Skips garbage.
        .collect::<VecDeque<usize>>();

    let mut expr = lines.next()?
        .strip_prefix("Operation: new = old ")?
        .split_ascii_whitespace()
        .take(2);

    let inspect = if let [Some(infix), Some(right)] = [expr.next(), expr.next()] {
        read_operation(infix, right)
    } else {
        None
    }?;

    let divisor = lines.next()?
        .strip_prefix("Test: divisible by ")?
        .parse::<usize>().ok()?;

    let success = lines.next()?
        .strip_prefix("If true: throw to monkey ")?
        .parse::<usize>().ok()?;

    let failure = lines.next()?
        .strip_prefix("If false: throw to monkey ")?
        .parse::<usize>().ok()?;

    Some(Monkey {
        _label: label,
        items,
        inspect,
        inspections: 0,
        throw: Throw {
            divisor,
            test: Box::new(move |old| old % divisor == 0),
            success,
            failure
        }
    })
}

fn read_operation(infix: &str, right: &str) -> Option<Box<dyn Fn(Worry) -> Worry>> {
        // Assumptions:
        // - The left operand is always `old`.
        // - The operand is addition or multiplication.

        let operands = match right {
            "old" => None,
            _ => right.parse::<usize>().ok(),
        };

        let operation: Option<Box<dyn Fn(Worry) -> Worry>> = match operands {
            None => match infix {
                "+" => Some(Box::new(|old| old + old)),
                "*" => Some(Box::new(|old| old * old)),
                _ => None,
            }
            Some(right) => match infix {
                "+" => Some(Box::new(move |old| old + right)),
                "*" => Some(Box::new(move |old| old * right)),
                _ => None,
            }
        };

        operation
}

#[test]
fn read_zeroth_monkey() {
    let zero_the_hero = r"Monkey 0:
        Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
          If true: throw to monkey 2
          If false: throw to monkey 3";

    let zero = read_monkey(zero_the_hero).unwrap();

    assert_eq!(zero.label, 0);
    assert_eq!(zero.items, VecDeque::from([79, 98]));

    assert_eq!((zero.inspect)(1), 19);
    assert_eq!((zero.inspect)(2), 38);

    assert!((zero.throw.test)(23));
    assert!((zero.throw.test)(46));
    assert!(!(zero.throw.test)(23 - 1));

    assert_eq!(zero.throw.success, 2);
    assert_eq!(zero.throw.failure, 3);
}

#[test]
fn read_second_monkey() {
    let zero_the_hero = r"Monkey 2:
        Starting items: 79, 60, 97
        Operation: new = old * old
        Test: divisible by 13
          If true: throw to monkey 1
          If false: throw to monkey 3";

    let two = read_monkey(zero_the_hero).unwrap();

    assert_eq!(two.label, 2);
    assert_eq!(two.items, VecDeque::from([79, 60, 97]));

    assert_eq!((two.inspect)(1), 1);
    assert_eq!((two.inspect)(2), 4);
    assert_eq!((two.inspect)(3), 9);

    assert!((two.throw.test)(13));
    assert!((two.throw.test)(26));
    assert!(!(two.throw.test)(13 - 1));

    assert_eq!(two.throw.success, 1);
    assert_eq!(two.throw.failure, 3);
}
