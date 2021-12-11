use std::cell::RefCell;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::rc::Rc;

type KnotHandle = Rc<RefCell<(isize, isize)>>;

#[derive(Debug)]
struct Segment {
    head: KnotHandle,
    tail: KnotHandle,
}

struct Rope {
    knots: Vec<KnotHandle>,
    segments: Vec<Segment>,
}

impl Rope {
    fn new(number_of_knots: usize) -> Option<Rope> {
        if number_of_knots > 1 {
            let mut segments = Vec::new();
            let mut knots = Vec::new();

            for _ in 1..=number_of_knots {
                knots.push(Rc::new(RefCell::new((0, 0))));
            }

            for head in 0..(number_of_knots - 1) {
                segments.push(Segment {
                    tail: knots[head].clone(),
                    head: knots[head + 1].clone(),
                });
            }

            return Some(Rope { knots, segments });
        }

        // A rope must have at least 2 knots and by implication at least one segment. The number
        // of segments is 1 less than the number of knots.
        None
    }

    fn tug(&mut self, tug: Tug) {
        self.segments.last_mut().unwrap().tug(tug);
        for segment in (0..(self.segments.len() - 1)).rev() {
            self.segments[segment].step();
        }
    }

    fn inspect(&self, knot: usize) -> (isize, isize) {
        *self.knots[knot].borrow()
    }

    fn head(&self) -> (isize, isize) {
        self.inspect(self.knots.len() - 1)
    }

    fn tail(&self) -> (isize, isize) {
        self.inspect(0)
    }
}

impl Segment {
    fn tug(&mut self, tug: Tug) {
        let (x, y) = *self.head.borrow();

        *self.head.borrow_mut() = match tug {
            Tug::Up    => (x,     y + 1),
            Tug::Right => (x + 1, y    ),
            Tug::Down  => (x,     y - 1),
            Tug::Left  => (x - 1, y    ),
        };

        self.step()
    }

    fn adjacent(&self) -> bool {
        let head = *self.head.borrow();
        let tail = *self.tail.borrow();

        (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
    }

    fn step(&mut self) {
        if !self.adjacent() {
            let head = *self.head.borrow();
            let (x, y) = *self.tail.borrow();

            let dx = (head.0 - x).signum();
            let dy = (head.1 - y).signum();
            *self.tail.borrow_mut() = (x + dx, y + dy);

            // Rope invariant: The head and tail are always adjacent.
            assert!(self.adjacent());
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Tug {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug)]
struct Motion {
    tug: Tug,
    times: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(Box::from("usage: cargo run -- puzzle-input"));
    }

    let puzzle = fs::read_to_string(&args[1])?;
    let motions = read_puzzle(&puzzle)?;

    println!("part one: {}", simulate(&motions, 2).len());
    println!("part two: {}", simulate(&motions, 10).len());

    Ok(())
}

fn simulate(motions: &Vec<Motion>, knots: usize) -> HashSet<(isize, isize)> {
    let mut rope = Rope::new(knots).unwrap();

    let mut tails: HashSet<(isize, isize)> = HashSet::new();
    tails.insert(rope.tail());

    for &Motion { tug, times } in motions {
        for _ in 1..=times {
            rope.tug(tug);
            tails.insert(rope.tail());
        }
    }

    tails
}

fn read_puzzle(puzzle: &str) -> Result<Vec<Motion>, Box<dyn Error>> {
    let mut motions = Vec::new();

    for line in puzzle.lines() {
        let mut motion = line.split_ascii_whitespace();
        let tug = motion.next()
            .expect("expect direction");
        let times = motion.next()
            .expect("expect times")
            .parse::<usize>()?;

        let tug = match tug {
            "U" => Ok(Tug::Up),
            "R" => Ok(Tug::Right),
            "D" => Ok(Tug::Down),
            "L" => Ok(Tug::Left),
            dir => Err(Box::<dyn Error>::from(
                format!("unknown direction '{}'", dir)
            )),
        }?;

        motions.push(Motion { tug, times })
    }

    Ok(motions)
}

#[test]
fn coiled() {
    let ropy = Rope::new(2).unwrap();
    assert_eq!(ropy.head(), (0, 0));
    assert_eq!(ropy.tail(), (0, 0));
}

#[test]
fn slack() {
    let mut ropy = Rope::new(2).unwrap();
    ropy.tug(Tug::Up);
    assert_eq!(ropy.head(), (0, 1));
    assert_eq!(ropy.tail(), (0, 0));
    ropy.tug(Tug::Right);
    assert_eq!(ropy.head(), (1, 1));
    assert_eq!(ropy.tail(), (0, 0));
    ropy.tug(Tug::Down);
    assert_eq!(ropy.head(), (1, 0));
    assert_eq!(ropy.tail(), (0, 0));
    ropy.tug(Tug::Down);
    assert_eq!(ropy.head(), (1, -1));
    assert_eq!(ropy.tail(), (0, 0));
}

#[test]
fn taut() {
    let mut ropy = Rope::new(2).unwrap();
    ropy.tug(Tug::Down);
    assert_eq!(ropy.head(), (0, -1));
    assert_eq!(ropy.tail(), (0, 0));
    ropy.tug(Tug::Down);
    assert_eq!(ropy.head(), (0, -2));
    assert_eq!(ropy.tail(), (0, -1));
    ropy.tug(Tug::Left);
    assert_eq!(ropy.head(), (-1, -2));
    assert_eq!(ropy.tail(), (0, -1));
    ropy.tug(Tug::Left);
    assert_eq!(ropy.head(), (-2, -2));
    assert_eq!(ropy.tail(), (-1, -2));
}
