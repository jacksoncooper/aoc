use std::error::Error;
use std::io::{stdin, Read, BufRead, BufReader};

//                              0  1  2  3  4  5  6  7  8  9
static SEGMENTS_IN: [u8; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

#[derive(Debug)]
struct Display
{
    digits: Vec<String>,
    display: Vec<String>
}

fn make_display(line: String) -> Display
{
    let mut halves = line.trim().split("|");

    // Panic if the input is not divided in two by the delimiter.
    let digits_string = halves.next().unwrap();
    let display_string = halves.next().unwrap();

    let digits: Vec<String> = digits_string
        .trim().split(" ").map(str::to_string).collect();
    let display: Vec<String> = display_string
        .trim().split(" ").map(str::to_string).collect();

    // Panic if the input does not contain ten digit patterns.
    if digits.len() != 10 { panic!("expect 10 digit patterns"); }

    // ...and four display digits.
    if display.len() != 4 { panic!("expect 4 display digits"); }

    Display { digits, display }
}

fn make_displays<R: Read>(mut reader: R) -> std::io::Result<Vec<Display>> {
    let mut displays = Vec::new();
    let buffered = BufReader::new(reader);
    for line in buffered.lines() {
        displays.push(make_display(line?));
    }
    Ok(displays)
}

fn part_one() -> std::io::Result<()>
{
    for display in make_displays(stdin())? {
        println!("{:#?}", display);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>
{
    part_one()?;
    Ok(())
}
