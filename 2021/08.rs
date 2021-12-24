use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::error::Error;
use std::io::{stdin, Read, BufRead, BufReader};

type Digit = HashSet<char>;

#[derive(Debug)]
struct Display
{
    digits: Vec<Digit>,
    display: Vec<Digit>
}

fn from_string(string: String) -> Digit
{
    let mut set = HashSet::new();
    for scalar in string.chars() {
        set.insert(scalar);
    }
    set
}

fn make_display(line: String) -> Display
{
    let mut halves = line.trim().split("|");

    // Panic if the input is not divided in two by the delimiter.
    let digits_string = halves.next().unwrap();
    let display_string = halves.next().unwrap();

    let digits: Vec<Digit> = digits_string
        .trim().split(" ").map(str::to_string).map(from_string).collect();
    let display: Vec<Digit> = display_string
        .trim().split(" ").map(str::to_string).map(from_string).collect();

    // Panic if the input does not contain ten digit patterns.
    if digits.len() != 10 { panic!("expect 10 digit patterns"); }

    // ...and four display digits.
    if display.len() != 4 { panic!("expect 4 display digits"); }

    Display { digits, display }
}

fn make_displays<R: Read>(reader: R) -> std::io::Result<Vec<Display>>
{
    let mut displays = Vec::new();
    let buffered = BufReader::new(reader);
    for line in buffered.lines() {
        displays.push(make_display(line?));
    }
    Ok(displays)
}

fn part_one() -> std::io::Result<u32>
{
    let mut to_known_digit: HashMap<usize, u8> = HashMap::new();
    to_known_digit.insert(2, 1);
    to_known_digit.insert(4, 4);
    to_known_digit.insert(3, 7);
    to_known_digit.insert(7, 8);

    let mut known_digits: u32 = 0;
    for display in make_displays(stdin())? {
        for digit in display.display.iter() {
            if let Some(segments) = to_known_digit.get(&digit.len()) {
                if vec![1, 4, 7, 8].contains(segments) {
                    known_digits += 1;
                }
            }
        }
    }

    Ok(known_digits)
}

fn part_two() -> std::io::Result<u32>
{
    let mut sum_of_readings: u32 = 0;

    for display in make_displays(stdin())? {
        // Sort digits by number of segments lit.
        let mut segments: HashMap<u32, Vec<&Digit>> = HashMap::new();

        for digit in display.digits.iter() {
            let segments_lit: u32 = u32::try_from(digit.len())
                .expect("at most 10 lit segments");

            match segments.get_mut(&segments_lit) {
                Some(list) => {
                    list.push(digit);
                },
                None => {
                    segments.insert(segments_lit, vec![digit]);
                }
            }
        }

        // Identify digits with unique number of segments.

        let one = &segments.get(&2).expect("expect 1")[0];
        let four = &segments.get(&4).expect("expect 4")[0];
        let seven = &segments.get(&3).expect("expect 7")[0];
        let eight = &segments.get(&7).expect("expect 8")[0];

        // Identify digits with 6 segments lit.

        let mut contains_one: Vec<&Digit> = Vec::new();
        let mut does_not_contain_one: Vec<&Digit> = Vec::new();

        for digit in segments.get(&6).expect("expect 0, 6, 9") {
            if one.intersection(digit).count() > 1 {
                contains_one.push(digit);
            } else {
                does_not_contain_one.push(digit);
            }
        }

        let six = does_not_contain_one[0];

        let maybe_zero = contains_one[0];
        let maybe_nine = contains_one[1];

        let zero: &Digit;
        let nine: &Digit;

        if four.intersection(maybe_nine).count() > 3 {
            zero = maybe_zero;
            nine = maybe_nine;
        } else {
            zero = maybe_nine;
            nine = maybe_zero;
        }

        // Identify digits with 5 segments lit.

        let mut contains_one: Vec<&Digit> = Vec::new();
        let mut does_not_contain_one: Vec<&Digit> = Vec::new();

        for digit in segments.get(&5).expect("expect 2, 3, 5") {
            if one.intersection(digit).count() > 1 {
                contains_one.push(digit);
            } else {
                does_not_contain_one.push(digit);
            }
        }

        let three = contains_one[0];

        let maybe_two = does_not_contain_one[0];
        let maybe_five = does_not_contain_one[1];

        let two: &Digit;
        let five: &Digit;

        if four.intersection(maybe_five).count() > 2 {
            two = maybe_two;
            five = maybe_five;
        } else {
            two = maybe_five;
            five = maybe_two;
        }

        // All digits discovered.

        let digits = [zero, one, two, three, four, five, six, seven, eight, nine];

        let mut reading: u32 = 0;
        for (power, digit) in (0..=3_u32).rev().zip(display.display.iter()) {
            for (out, candidate) in (0..=9_u32).zip(digits.iter().cloned()) {
                if digit == candidate {
                    reading += out * 10_u32.pow(power);
                }
            }
        }

        sum_of_readings += reading;
    }

    Ok(sum_of_readings)
}

fn main() -> Result<(), Box<dyn Error>>
{
    // let known_digits = part_one()?;
    // println!("There are {} known digits displayed on the submarine's screens.", known_digits);
    let sum_of_readings = part_two()?;
    println!("The sum of the four-digit numbers displayed on the submarine's panels is {}.", sum_of_readings);
    Ok(())
}
