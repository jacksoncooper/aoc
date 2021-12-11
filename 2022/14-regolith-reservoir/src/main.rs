use caves::Cave;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: cargo run -- puzzle-input");
        std::process::exit(1);
    }

    let mut cave: Cave = std::fs::read_to_string(&args[1])
        .unwrap()  // <-+
        .parse()   //   |- it's Christmas morning
        .unwrap(); // <-+

    let lots = cave.drop_lots(false);
    println!("part one: {}", lots);

    let mut cave: Cave = std::fs::read_to_string(&args[1])
        .unwrap()  // <-+
        .parse()   //   |- ditto
        .unwrap(); // <-+

    let lots = cave.drop_lots(true);
    // println!("{}", cave);
    println!("part two: {}", lots);
}

mod caves
{
    use std::fmt::Display;
    use std::str::FromStr;

    use super::grids::Grid;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Cell {
        Air,
        Rock,
        Sand,
    }

    impl Default for Cell {
        fn default() -> Self {
            Cell::Air
        }
    }

    impl Display for Cell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // TODO: Does Rust follow references in match expressions?
            write!(f, "{}", match self {
                Cell::Air =>  '.',
                Cell::Rock => '#',
                Cell::Sand => 'o'
            })
        }
    }

    pub struct Cave {
        grid: Grid<Cell>,
        source: (usize, usize),
    }

    fn parse_path(line: &str) -> Vec<(usize, usize)> {
        line.trim()
            .split(" -> ")
            // Danger! Ignores malformed coordinates.
            .flat_map(|coord| {
                let mut tuple = coord.split(',');
                let column = tuple.next()?.parse::<usize>().ok()?;
                let row = tuple.next()?.parse::<usize>().ok()?;
                Some((row, column))
            })
            .collect()
    }

    fn inflate_path(path: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut inflated = Vec::new();

        for coord in 1..path.len() {
            let head = path[coord - 1];
            let tail = path[coord];

            let row_constant;
            let constant;
            let range;

            // > each point indicates the end of a straight horizontal or vertical line to be
            // > drawn from the previous point

            // Rows increase towards abyss, columns decrease towards zero.

            if head.0 == tail.0 { // Horizontal.
                row_constant = true;
                constant = head.0;
                range = (head.1, tail.1);
            } else { // Vertical.
                assert_eq!(head.1, tail.1);
                row_constant = false;
                constant = head.1;
                range = (head.0, tail.0);
            }

            // TODO: Whee heap allocation because stack references to trait objects are annoying.
            let range: Box<dyn Iterator<Item=usize>> = if range.0 < range.1 {
                // Inclusive of smallest value, exclusive of largest.
                Box::new(range.0..range.1)
            } else {
                // Exclusive of smallest value, inclusive of largest.
                Box::new(((range.1 + 1)..=(range.0)).rev())
            };

            for offset in range {
                if row_constant {
                    inflated.push((constant, offset));
                } else {
                    inflated.push((offset, constant));
                }
            }
        }

        let last = path[path.len() - 1];
        inflated.push(last);

        inflated
    }

    impl Cave {
        fn drop(&mut self) -> Option<(usize, usize)> {
            let mut grain = self.source;

            // The source is blocked!
            if *self.grid.get(self.source).unwrap() != Cell::Air {
                return None;
            }

            loop {
                // If (grain.0 + 1, grain.1) is off the grid, so is any other possible path.
                while ![Cell::Rock, Cell::Sand].contains(self.grid.get((grain.0 + 1, grain.1))?) {
                    grain = (grain.0 + 1, grain.1);
                }

                // If (grain.0 + 1, grain.1 - 1) is off the grid, so is any other possible path.
                if ![Cell::Rock, Cell::Sand].contains(self.grid.get((grain.0 + 1, grain.1 - 1))?) {
                    grain = (grain.0 + 1, grain.1 - 1);
                    continue;
                }

                // If (grain.0 + 1, grain.1 + 1) is off the grid, so is any other possible path.
                if ![Cell::Rock, Cell::Sand].contains(self.grid.get((grain.0 + 1, grain.1 + 1))?) {
                    grain = (grain.0 + 1, grain.1 + 1);
                    continue;
                }

                // The grain is stuck!
                self.grid.put(grain, Cell::Sand);
                return Some(grain);
            }
        }

        pub fn oops_theres_a_floor(&mut self) {
            let (height, width) = self.grid.size();
            let floor_depth = (height - 1) + 2;
            for column in 0..width {
                self.grid.put((floor_depth, column), Cell::Rock);
            }
        }

        pub fn drop_lots(&mut self, oops_floor: bool) -> usize {
            // TODO: Silly hack because the sand is rolling out of the map due to the tight bounds
            // of the "dynamic" grid.

            if oops_floor {
                let wiggle_room = 200;
                let (height, width) = self.grid.size();
                self.grid.put((height - 1, (width - 1) + wiggle_room), Cell::Air);
                self.oops_theres_a_floor();
            }

            let mut lots = 0;

            while let Some(_) = self.drop() {
                lots += 1;
            }

            lots
        }
    }

    impl FromStr for Cave {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut grid = Grid::new();

            let paths = s.lines().map(|path| inflate_path(parse_path(path)));
            for path in paths {
                for coord in path {
                    grid.put(coord, Cell::Rock);
                }
            }

            Ok(Cave {
                grid,
                source: (0, 500),
            })
        }
    }

    impl Display for Cave {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.grid)
        }
    }

    #[test]
    fn parse_first_line_of_sample() {
        assert_eq!(
            parse_path("498,4 -> 498,6 -> 496,6"),
            vec![(4, 498), (6, 498), (6, 496)]
        );
    }

    #[test]
    fn parse_second_line_of_sample() {
        assert_eq!(
            parse_path("503,4 -> 502,4 -> 502,9 -> 494,9"),
            vec![(4, 503), (4, 502), (9, 502), (9, 494)]
        )
    }

    #[test]
    fn inflate_first_line_of_sample() {
        assert_eq!(
            inflate_path(parse_path("498,4 -> 498,6 -> 496,6")),
            vec![(4, 498), (5, 498), (6, 498), (6, 497), (6, 496)]
        );
    }

    #[test]
    fn inflate_second_line_of_sample() {
        assert_eq!(
            inflate_path(parse_path("503,4 -> 502,4 -> 502,9 -> 494,9")),
            vec![(4, 503), (4, 502), (5, 502), (6, 502), (7, 502), (8, 502), (9, 502),
                 (9, 501), (9, 500), (9, 499), (9, 498), (9, 497), (9, 496), (9, 495), (9, 494)]
        )
    }

    #[test]
    fn drop_three_from_sample() -> Result<(), ()> {
        let mut cave = Cave::from_str(
            "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9"
        )?;

        assert_eq!(cave.drop(), Some((8, 500)));

        Ok(())
    }
}

mod grids
{
    use std::fmt::Display;

    pub struct Grid<T> {
        height: usize,
        width: usize,
        cells: Vec<T>,
    }

    impl<T> Grid<T> {
        pub fn new() -> Grid<T> {
            Grid { width: 0, height: 0, cells: Vec::new() }
        }

        pub fn size(&self) -> (usize, usize) {
            (self.height, self.width)
        }

        pub fn get(&self, (row, column): (usize, usize)) -> Option<&T> {
            if let Some(offset) = self.to_offset((row, column)) {
                return Some(&self.cells[offset]);
            }

            None
        }

        fn to_offset(&self, (row, column): (usize, usize)) -> Option<usize> {
            let offset = self.width * row + column;
            (offset < self.cells.len()).then_some(offset)
        }
    }

    impl<T: Clone + Default> Grid<T> {
        pub fn put(&mut self, (row, column): (usize, usize), entry: T) {
            // Insertions outside of the bounds drop the grid and resize it. This is extremely
            // expensive and unoptimized.

            if self.height <= row || self.width <= column {
                let new_height = if self.height <= row  { row + 1 } else { self.height };
                let new_width = if self.width <= column { column + 1 } else { self.width };
                let new_length = new_height * new_width;

                let mut bigger_buffer = Vec::with_capacity(new_length);

                for _ in 0..new_length {
                    bigger_buffer.push(T::default());
                }

                let mut bigger_grid = Grid {
                    height: new_height,
                    width: new_width,
                    cells: bigger_buffer,
                };

                for row in 0..self.height {
                    for column in 0..self.width {
                        let entry = self.get((row, column)).unwrap().clone();
                        bigger_grid.put((row, column), entry); // TODO: Move out of vector, darn it.
                    }
                }

                // TODO: Does *self = bigger_grid move?
                *self = bigger_grid;
            }

            let offset = self.to_offset((row, column));
            self.cells[offset.unwrap()] = entry;
        }
    }

    impl<T: Display> Grid<T> {
        fn display(&self) -> String {
            let mut lines = Vec::new();

            for row in 0..self.height {
                let mut line = String::new();
                for column in 0..self.width {
                    line.push_str(&self.get((row, column)).unwrap().to_string());
                }
                lines.push(line);
            }

            lines.join("\n")
        }
    }

    impl<T: Display> Display for Grid<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            return write!(f, "{}", self.display())
        }
    }

    #[test]
    fn prod_small_grid() {
        let mut my_squares = Grid::new();

        my_squares.put((0, 2), 'a');
        my_squares.put((1, 1), 'b');

        assert_eq!(my_squares.size(), (2, 3));
        assert_eq!(my_squares.get((1, 1)), Some(&'b'));
        assert_eq!(my_squares.get((0, 2)), Some(&'a'));
        assert_eq!(my_squares.get((2, 2)), None);

        // 0 1 2
        // . . a 0
        // . b . 1
        //     x
    }
}
