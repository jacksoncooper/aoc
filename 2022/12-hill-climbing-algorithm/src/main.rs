use std::collections::{HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fs;
use std::str::FromStr;

type Position = usize;

struct Grid {
    width: usize,
    height: usize,
    start: Position,
    goal: Position,
    squares: Vec<u8>,
}

enum Step {
    North, South, East, West
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(Box::from("usage: cargo run -- puzzle-input"));
    }

    let puzzle = fs::read_to_string(&args[1])?;

    let grid = puzzle.parse::<Grid>()?;

    let to_summit = hike(&grid, grid.start);

    println!("part one: {:?}", to_summit);

    // Can't refactor to reverse the breadth-first search I'm sorry core.

    let to_summit = grid.elevation(0).iter()
        .flat_map(|&start| hike(&grid, start))
        .min();

    println!("part two: {:?}", to_summit);

    Ok(())
}

fn hike(grid: &Grid, start: Position) -> Option<usize> {
    let mut discovered: VecDeque<(usize, Position)> = VecDeque::from([(0, start)]);
    let mut seen: HashSet<Position> = HashSet::from([start]);

    while let Some((distance, position)) = discovered.pop_front() {
        if position == grid.goal {
            return Some(distance);
        }

        let adjacent =
            [ grid.step(position, Step::North),
              grid.step(position, Step::East),
              grid.step(position, Step::South),
              grid.step(position, Step::West),
            ].into_iter()
            .filter_map(|position| position)
            .filter(|position| !seen.contains(position))
            .collect::<Vec<Position>>();

        seen.extend(&adjacent);

        discovered.extend(
            adjacent.into_iter()
            .map(|position| (distance + 1, position))
        );
    }

    None
}

impl Grid {
    fn offset(&self, (row, column): (usize, usize)) -> Option<Position> {
        let offset = self.width * row + column;
        self.valid_offset(offset).then_some(offset)
    }

    fn valid_offset(&self, offset: usize) -> bool {
        offset < self.height * self.width
    }

    fn coordinates(&self, offset: usize) -> Option<(usize, usize)> {
        let offset = self.valid_offset(offset).then_some(offset)?;
        Some((offset / self.width, offset % self.width))
    }

    fn step(&self, offset: usize, step: Step) -> Option<Position> {
        let (row, column) = self.coordinates(offset)?;

        // TODO: Yikes. I have all the stack space in the world please don't yell at me.
        let north = (row > 0).then(|| self.offset((row - 1, column)).unwrap());
        let east  = self.offset((row, column + 1));
        let south = self.offset((row + 1, column));
        let west  = (column > 0).then(|| self.offset((row, column - 1)).unwrap());

        match step {
            Step::North if self.lower(offset, north?) => north,
            Step::East  if self.lower(offset, east?)  => east,
            Step::South if self.lower(offset, south?) => south,
            Step::West  if self.lower(offset, west?)  => west,
            _ => None,
        }
    }

    fn lower(&self, from: usize, to: usize) -> bool {
        self.squares[from] + 1 >= self.squares[to]
    }

    fn elevation(&self, desired: u8) -> Vec<Position> {
        self.squares.iter()
            .enumerate()
            .filter_map(|(position, &elevation)| (desired == elevation).then_some(position))
            .collect()
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let width = value.lines().next().ok_or("expect heightmap".to_string())?.len();
        let mut squares = Vec::new();

        let mut height_offset = 0;
        let mut offset: usize = 0;

        let mut start = None;
        let mut goal = None;

        for line in value.lines() {
            for symbol in line.chars() {
                match symbol {
                    'S' => {
                        start = match start {
                            None => Some(offset),
                            Some(found) => return Err(
                                format!("expect one 'S', found at offsets {} and {}", found, offset)
                            ),
                        };
                        squares.push(b'a' - b'a')
                    }
                    'E' => {
                        goal = match goal {
                            None => Some(offset),
                            Some(found) => return Err(
                                format!("expect one 'E', found at offsets {} and {}", found, offset)
                            ),
                        };
                        squares.push(b'z' - b'a')
                    }
                     _  =>
                        squares.push(
                            if ('a'..='z').contains(&symbol) {
                                symbol as u8 - b'a'
                            } else {
                                Err(format!("unexpected elevation '{}'", symbol))?
                            }
                        )
                }
                offset += 1;
            }
            height_offset += 1;
        }

        let start = if start.is_none() {
            Err("expect 'S'".to_string())?
        } else {
            start.unwrap()
        };

        let goal = if goal.is_none() {
            Err("expect 'E'".to_string())?
        } else {
            goal.unwrap()
        };

        return Ok(Grid {
            width, height: height_offset,
            start, goal,
            squares
        })
    }
}

#[test]
fn small_grid() -> Result<(), String> {
    let cutie = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    //  a  b  c  d  e  f  g  h  i  j  k  l  m  n  o  p  q  r  s  t  u  v  w  x  y  z
    // 00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25

    let grid = cutie.parse::<Grid>()?;

    assert_eq!(grid.width, 8);
    assert_eq!(grid.height, 5);
    assert_eq!(grid.start, 0);
    assert_eq!(grid.goal, 21);

    //       S  a  b   q   p   o   n   m
    if let &[0, 0, 1, 16, 15, 14, 13, 12, ..] = &grid.squares[..] {
        Ok(())
    } else {
        Err(format!("unexpected grid: {:?}", &grid.squares[0..grid.width]))
    }
}

#[test]
fn walk_small_grid() -> Result<(), String> {
    let cutie = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    let grid = cutie.parse::<Grid>()?;

    let mut square = grid.start;

    assert_eq!(grid.step(square, Step::North), None);

    square = grid.step(square, Step::East).unwrap();
    assert_eq!(grid.coordinates(square), Some((0, 1)));

    square = grid.step(square, Step::South).unwrap();
    assert_eq!(grid.coordinates(square), Some((1, 1)));

    square = grid.step(square, Step::South).unwrap();
    assert_eq!(grid.coordinates(square), Some((2, 1)));

    square = grid.step(square, Step::West).unwrap();
    assert_eq!(grid.coordinates(square), Some((2, 0)));

    assert_eq!(grid.step(square, Step::West), None);

    for row in 3..=4 {
        square = grid.step(square, Step::South).unwrap();
        assert_eq!(grid.coordinates(square), Some((row, 0)));
    }

    square = grid.step(square, Step::East).unwrap();
    assert_eq!(grid.coordinates(square), Some((4, 1)));

    assert_eq!(grid.step(square, Step::East), None);

    Ok(())
}
