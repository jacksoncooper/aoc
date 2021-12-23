use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>>
{
    let mut line = String::new();
    BufReader::new(stdin()).read_line(&mut line)?;
    let counts = make_counts(line)?;
    let (position, fuel) = minimum_fuel_for_maneuver(&counts, true);
    println!("🦀s go to position {} using {} fuel.", position, fuel);
    Ok(())
}

fn make_counts(line: String) -> Result<HashMap<u32, u32>, <u32 as FromStr>::Err>
{
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for crab in line.trim().split(",") {
        match crab.parse::<u32>() {
            Ok(position) => {
                match counts.get_mut(&position) {
                    Some(frequency) => *frequency += 1,
                    None => {
                        counts.insert(position, 1);
                    }
                }
            },
            Err(error) => return Err(error),
        }
    }
    return Ok(counts);
}

fn minimum_fuel_for_maneuver(counts: &HashMap<u32, u32>, part_one: bool) -> (u32, u32)
{
    // Using ::MAX isn't idiomatic, but the alternative is to use an Option
    // type, which would require more syntax.

    let mut final_cost: u32 = u32::MAX;
    let mut final_position: u32 = 0;

    // Panic if no 🦀.
    let frontier_crab: u32 = *counts.keys().max().unwrap();

    for ref formation_position in 0..=frontier_crab {
        let maneuver_cost: u32 = fuel_for_maneuver(counts, formation_position, part_one);
        if maneuver_cost < final_cost {
            final_cost = maneuver_cost;
            final_position = *formation_position;
        }
    }

    (final_position, final_cost)
}

fn fuel_for_maneuver(counts: &HashMap<u32, u32>, formation_position: &u32, part_one: bool) -> u32
{
    let mut maneuver_cost: u32 = 0;

    for (position, count) in counts {
        let distance: u32 = if position > formation_position {
            position - formation_position
        } else {
            formation_position - position
        };

        let fuel: u32 = if part_one {
            distance
        } else {
            distance * (distance + 1) / 2
        };

        maneuver_cost += count * fuel;
    }

    maneuver_cost
}
