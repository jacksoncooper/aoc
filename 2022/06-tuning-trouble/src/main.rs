use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(Box::from("usage: cargo run -- puzzle-input"));
    }

    let stream = read_puzzle(&args[1])?;

    // TODO: Danger, relies on the fact that the Unicode UTF-8 encoding of ASCII characters is
    // ASCII.
    let part_one = 1 + seek_start_of_packet(stream.as_bytes(), 4)
        .ok_or(Box::<dyn Error>::from("expect start of packet marker"))?;

    let part_two = 1 + seek_start_of_packet(stream.as_bytes(), 14)
        .ok_or(Box::<dyn Error>::from("expect start of message marker"))?;

    println!("part one: {}", part_one);
    println!("part two: {}", part_two);

    Ok(())
}

fn seek_start_of_packet(stream: &[u8], width: usize) -> Option<usize> {
    let mut window: HashMap<u8, usize> = HashMap::new();

    for offset in 0..stream.len() {
        let head = stream[offset];

        let tail = if offset >= width {
            Some(stream[offset - width])
        } else { None };

        let count = window.entry(head).or_insert(0);
        *count += 1;

        if let Some(tail) = tail {
            let count = window.get_mut(&tail)
                .expect("expect subscript to be tallied");

            if *count == 1 {
                window.remove(&tail);
            } else {
                *count -= 1;
            }
        }

        if window.len() == width {
            return Some(offset);
        }
    }

    None
}

fn read_puzzle(puzzle: &str) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(puzzle)?)
}

#[cfg(test)]
mod tests {
    use crate::seek_start_of_packet;

    #[test]
    fn marker_at_start() {
        let stream = "abcd".as_bytes();
        assert_eq!(seek_start_of_packet(stream, 4), Some(3));
    }

    #[test]
    fn marker_at_furthest_offset() {
        let stream = "abddefg".as_bytes();
        assert_eq!(seek_start_of_packet(stream, 4), Some(6));
    }

    #[test]
    fn first_more_example() {
        let stream = "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes();
        assert_eq!(seek_start_of_packet(stream, 4), Some(4));
    }

    #[test]
    fn second_more_example() {
        let stream = "nppdvjthqldpwncqszvftbrmjlhg".as_bytes();
        assert_eq!(seek_start_of_packet(stream, 4), Some(5));
    }

    #[test]
    fn third_more_example() {
        let stream = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes();
        assert_eq!(seek_start_of_packet(stream, 4), Some(9));
    }

    #[test]
    fn fourth_more_example() {
        let stream = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes();
        assert_eq!(seek_start_of_packet(stream, 4), Some(10));
    }
}
