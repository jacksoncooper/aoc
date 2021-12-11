use std::cmp::Ord;
use std::cmp::Ordering::{self, *};
use std::env;
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Packet(Vec<Entry>);

#[derive(Clone, Debug, PartialEq, Eq)]
enum Entry {
    List(Packet),
    Int(u32),
}

use self::Entry::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(Box::from("usage: cargo run -- puzzle-input"));
    }

    let puzzle = fs::read_to_string(&args[1])?;

    let part_one = in_order_pairs(&puzzle)
        .ok_or(Box::<dyn Error>::from("parse error".to_string()))?;

    println!("part one: {}", part_one.into_iter().sum::<usize>());

    println!("part two: {}", decoder_key(&puzzle));

    Ok(())
}

fn in_order_pairs(puzzle: &str) -> Option<Vec<usize>> {
    let mut indices = Vec::new();

    for (index, pair) in (1..).zip(puzzle.split("\n\n")) {
        let mut pair = pair.lines();

        let left = pair.next()?.parse::<Packet>().ok()?;
        let right = pair.next()?.parse::<Packet>().ok()?;

        if left.in_order(&right) {
            indices.push(index);
        }
    }

    Some(indices)
}

fn decoder_key(puzzle: &str) -> usize {
    let two = Packet(vec![List(Packet(vec![Int(2)]))]);
    let six = Packet(vec![List(Packet(vec![Int(6)]))]);
    let mut packets = vec![two.clone(), six.clone()];

    // Interpret all lines as a packet and fail on empty ones.
    for line in puzzle.lines() {
        if let Ok(packet) = line.parse::<Packet>() {
            packets.push(packet);
        }
    }

    // Assume no packets are equal.
    packets.sort_unstable();

    // Unwrap because we've added the divider packets to the list.
    let first_divider = packets.iter().position(|packet| *packet == two).unwrap() + 1;
    let second_divider = packets.iter().position(|packet| *packet == six).unwrap() + 1;

    first_divider * second_divider
}

impl Packet {
    fn in_order(&self, other: &Self) -> bool {
        [Less, Equal].contains(&self.compare(other))
    }

    fn compare(&self, right: &Packet) -> Ordering {
        // TODO: Is `in_order` is transitive?

        let mut left = self.0.iter();
        let mut right = right.0.iter();

        loop {
            match (left.next(), right.next()) {
                (Some(List(left)), Some(List(right))) => {
                    let order = left.compare(right);
                    if order != Equal {
                        return order
                    }
                }
                (Some(&Int(left_int)), Some(List(right))) => {
                    let order = Packet(vec![Int(left_int)]).compare(right);
                    if order != Equal {
                        return order
                    }
                }
                (Some(List(left)), Some(&Int(right_int))) => {
                    let order = left.compare(&Packet(vec![Int(right_int)]));
                    if order != Equal {
                        return order
                    }
                }
                (Some(Int(left_int)), Some(Int(right_int))) => {
                    let order = left_int.cmp(&right_int);
                    if order != Equal {
                        return order
                    }
                }
                (None, Some(_)) =>
                    return Less,
                (Some(_), None) =>
                    return Greater,
                (None, None) =>
                    return Equal,
            }
        }
    }
}

impl PartialOrd<Packet> for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.compare(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let symbols = s.strip_prefix('[')
            .ok_or("expect '[' at start of packet")?
            .strip_suffix(']')
            .ok_or("expect ']' at end of packet")?;

        let mut symbols = symbols.chars();

        fn go<I: Iterator<Item=char>>(symbols: &mut I) -> Result<Packet, String> {
            // Parse the interior of a packet.
            let mut entries: Vec<Entry> = Vec::new();

            while let Some(sym) = symbols.next() {
                match sym {
                    '[' => {
                        let packet = go(symbols)?;
                        entries.push(List(packet));
                    }
                    ',' => (),
                    ']' => return Ok(Packet(entries)),
                    int => {
                        let mut digits = int.to_string();

                        // `peekable` consumes the iterator on the first call so I sloppily
                        // implemented lookahead with this Boolean.
                        let mut last_digit = false;

                        while let Some(sym) = symbols.next() {
                            match sym {
                                ',' => {
                                    break;
                                },
                                ']' => {
                                    last_digit = true;
                                    break;
                                },
                                dig => {
                                    digits.push(dig);
                                }
                            }
                        }

                        let int = digits.parse::<u32>()
                            .map_err(|_| format!("'{}' is not an integer", sym))?;

                        entries.push(Entry::Int(int));

                        if last_digit {
                            return Ok(Packet(entries));
                        }
                    }
                }
            }

            Ok(Packet(entries))
        }

        go(&mut symbols)
    }

}

#[test]
fn read_first_sample_packet() -> Result<(), String> {
    assert_eq!(
        "[1,1,3,1,1]".parse::<Packet>()?,
        Packet(vec![Int(1), Int(1), Int(3), Int(1), Int(1)]),
    );

    Ok(())
}

#[test]
fn read_empty_packet() -> Result<(), String> {
    assert_eq!(
        "[]".parse::<Packet>()?,
        Packet(vec![]),
    );

    Ok(())
}

#[test]
fn read_singleton() -> Result<(), String> {
    assert_eq!(
        "[1]".parse::<Packet>()?,
        Packet(vec![Int(1)]),
    );

    Ok(())
}

#[test]
fn read_third_sample_packet() -> Result<(), String> {
    assert_eq!(
        "[[1],[2,3,4]]".parse::<Packet>()?,
        Packet(vec![List(Packet(vec![Int(1)])), List(Packet(vec![Int(2), Int(3), Int(4)]))]),
    );

    Ok(())
}

#[test]
fn read_thirteenth_sample_packet() -> Result<(), String> {
    assert_eq!(
        "[[[]]]".parse::<Packet>()?,
        Packet(vec![List(Packet(vec![List(Packet(vec![]))]))]),
    );

    Ok(())
}

#[test]
fn compare_first_sample_pair() -> Result<(), String> {
    assert!("[1,1,3,1,1]".parse::<Packet>()?.in_order( &"[1,1,5,1,1]".parse::<Packet>()?));
    Ok(())
}

#[test]
fn compare_second_sample_pair() -> Result<(), String> {
    let left = "[[1],[2,3,4]]".parse::<Packet>()?;
    let right = "[[1],4]".parse::<Packet>()?;

    assert_eq!(
        right,
        Packet(vec![List(Packet(vec![Int(1)])), Int(4)])
    );

    assert_eq!(
        left,
        Packet(vec![List(Packet(vec![Int(1)])), List(Packet(vec![Int(2), Int(3), Int(4)]))])
    );

    assert!(left.in_order(&right));

    Ok(())
}

#[test]
fn compare_singleton_pair() -> Result<(), String> {
    assert!("[1]".parse::<Packet>()?.in_order(&"[1]".parse::<Packet>()?));
    Ok(())
}

#[test]
fn compare_promoted_pair() -> Result<(), String> {
    let left = "[[2,3,4]]".parse::<Packet>()?;
    let right = "[4]".parse::<Packet>()?;

    assert_eq!(
        left,
        Packet(vec![List(Packet(vec![Int(2), Int(3), Int(4)]))])
    );

    assert_eq!(
        right,
        Packet(vec![Int(4)])
    );

    assert!(left.in_order(&right));

    Ok(())
}

#[test]
fn compare_shorter_pair() -> Result<(), String> {
    assert!("[1,2,3]".parse::<Packet>()?.in_order(&"[1,2,3,4,5]".parse::<Packet>()?));
    Ok(())
}

#[test]
fn compare_longer_pair() -> Result<(), String> {
    assert!(!"[1,2,3,4,5,6]".parse::<Packet>()?.in_order(&"[1,2,3,4,5]".parse::<Packet>()?));
    Ok(())
}

#[test]
fn compare_same_length_pair() -> Result<(), String> {
    assert!("[1,2,3,4,5,6]".parse::<Packet>()?.in_order(&"[1,2,3,4,5,6]".parse::<Packet>()?));
    Ok(())
}

#[test]
fn compare_third_sample_pair() -> Result<(), String> {
    assert!(!"[9]".parse::<Packet>()?.in_order(&"[[8,7,6]]".parse::<Packet>()?));
    Ok(())
}

#[test]
fn compare_fourth_sample_pair() -> Result<(), String> {
    assert!("[[4,4],4,4]".parse::<Packet>()?.in_order(&"[[4,4],4,4,4]".parse::<Packet>()?));
    Ok(())
}
#[test]
fn compare_fifth_sample_pair() -> Result<(), String> {
    assert!(!"[7,7,7,7]".parse::<Packet>()?.in_order(&"[7,7,7]".parse::<Packet>()?));
    Ok(())
}

#[test]
fn compare_sixth_sample_pair() -> Result<(), String> {
    assert!("[]".parse::<Packet>()?.in_order(&"[3]".parse::<Packet>()?));
    Ok(())
}

#[test]
fn compare_seventh_sample_pair() -> Result<(), String> {
    assert!(!"[[[]]]".parse::<Packet>()?.in_order(&"[[]]".parse::<Packet>()?));
    Ok(())
}

#[test]
fn compare_eighth_sample_pair() -> Result<(), String> {
    assert!(!"[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Packet>()?
        .in_order(&"[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Packet>()?));
    Ok(())
}

#[test]
fn compare_nested_success() -> Result<(), String> {
    assert!("[[1],6]".parse::<Packet>()?.in_order(&"[[2],4]".parse::<Packet>()?));
    Ok(())
}
