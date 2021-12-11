use std::error::Error;
use std::io::{stdin, Read, BufRead, BufReader};
use std::str::FromStr;

const NOC: usize = 9;          // Number of clocks.
const DTM: usize = 2;          // Days to maturity.
type Clocks      = [u64; NOC]; // The fishies.

fn main() -> Result<(), Box<dyn Error>>
{
    let days: u64 = 256;
    let population: u64 = simulate(stdin(), days)?;
    println!("After {} days, there are {} lanternfish.", days, population);
    Ok(())
}

fn make_clocks(line: String) -> Result<Clocks, <u64 as FromStr>::Err>
{
    let mut clocks: Clocks = [0; NOC];
    for clock in line.trim().split(",") {
        match clock.parse::<usize>() {
            Ok(days_left) => clocks[days_left] += 1,
            Err(error) => return Err(error)
        }
    }
    Ok(clocks)
}

fn simulate<R: Read>(reader: R, days: u64) -> Result<u64, Box<dyn Error>>
{
    // [0, 1, 2, 3, 4, 5, 6, 7, 8]
    //                 ^  |     ^
    //     time to spawn  |     time to mature

    let mut line = String::new();
    BufReader::new(reader).read_line(&mut line)?;
    let mut clocks = make_clocks(line)?;

    for _ in 0..days {
        let ready_to_spawn: u64 = clocks[0];
        for days_left in 0..NOC - 1 {
            clocks[days_left] = clocks[days_left + 1];
        }
        clocks[NOC - DTM - 1] += ready_to_spawn;
        clocks[NOC - 1] = ready_to_spawn;
    }

    Ok(clocks.iter().sum())
}
