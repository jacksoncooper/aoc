use std::collections::HashSet;
use std::cmp;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(Box::from("usage: cargo run -- puzzle-input"));
    }

    let grove = read_puzzle(&args[1])?;
    let visible = visible_trees(&grove);

    println!("part one: {:?}", visible.len());
    println!("part two: {:?}", part_two(&grove));

    Ok(())
}

fn part_two(grove: &Vec<Vec<u8>>) -> usize {
    let rows = grove.len();
    let columns = grove[0].len();

    let mut total_score = 0;

    for row in 0..rows {
        for column in 0..columns {
            total_score = cmp::max(total_score, scenic_score(grove, (row, column)))
        }
    }

    total_score
}

fn visible_trees(grove: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    let rows = grove.len();
    let columns = grove[0].len();

    for (row_distance, row) in grove.iter().enumerate() {
        let column_distances = look(row.iter());
        visible.extend(column_distances.iter().map(|&distance| (row_distance, distance)));
        let column_distances = look(row.iter().rev());
        visible.extend(column_distances.iter().map(|&distance| (row_distance, columns - distance - 1)));
    }

    for column_distance in 0..columns {
        let column = (0..rows).map(|distance| grove[distance][column_distance]).collect::<Vec<u8>>();
        let row_distances = look(column.iter());
        visible.extend(row_distances.iter().map(|&distance| (distance, column_distance)));
        let row_distances = look(column.iter().rev());
        visible.extend(row_distances.iter().map(|&distance| (rows - distance - 1, column_distance)));
    }

    visible
}

fn look<'a, I: Iterator<Item=&'a u8>>(mut trees: I) -> Vec<usize> {
    let mut visible = Vec::new();

    // The nearest tallest tree. The leftmost tallest tree?
    let mut tallest = if let Some(tree) = trees.next() {
        tree
    } else {
        return visible;
    };

    visible.push(0);

    for (tree, distance) in trees.zip(1usize..) {
        if tree > tallest {
            visible.push(distance);
            tallest = tree;
        }
    }

    visible
}

fn scenic_score(grove: &Vec<Vec<u8>>, tree: (usize, usize)) -> usize {
    let rows = grove.len();
    let columns = grove[0].len();

    let (tree_row, tree_column) = tree;
    let tree_house = grove[tree_row][tree_column];

    let mut total_score = 1;

    let offset = shorter_trees(grove[tree_row].iter().skip(tree_column + 1), tree_house);
    let mut score = offset;
    if usize::from(tree_column) + offset < rows - 1 {
        score += 1;
    }
    total_score *= score;

    let offset = shorter_trees(grove[tree_row].iter().take(tree_column).rev(), tree_house);
    let mut score = offset;
    if usize::from(tree_column) - offset > 0 {
        score += 1;
    }
    total_score *= score;

    let column = (0..rows).map(|row| grove[row][tree_column]).collect::<Vec<u8>>();

    let offset = shorter_trees(column.iter().skip(tree_row + 1), tree_house);
    let mut score = offset;
    if usize::from(tree_row) + offset < columns - 1 {
        score += 1;
    }
    total_score *= score;

    let offset = shorter_trees(column.iter().take(tree_row).rev(), tree_house);
    let mut score = offset;
    if usize::from(tree_row) - offset > 0 {
        score += 1;
    }
    total_score *= score;

    total_score
}

fn shorter_trees<'a, I: Iterator<Item=&'a u8>>(trees: I, tree_house: u8) -> usize {
    trees.take_while(|&tree| tree_house > *tree).count()
}

fn read_puzzle(puzzle: &str) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    let puzzle = fs::read_to_string(puzzle)?;

    let rows = puzzle.lines()
        .map(|row| row.chars()
            .map(|char| char.to_string().parse::<u8>())
            // TODO: This fold is horrific. Where are my monads.
            .fold(Ok(Vec::new()), |acc, maybe_digit| {
                match acc {
                    Ok(mut digits) => {
                        match maybe_digit {
                            Ok(digit) => {
                                digits.push(digit);
                                Ok(digits)
                            },
                            Err(err) => Err(err),
                        }
                    },
                    err => err,
                }
            })
        );

    // TODO. I give up.
    // Vec<Result<Vec<u8>, ParseIntError>> -> Result<Vec<Vec<u8>>, ParseIntError>

    let mut grove = Vec::new();
    for row in rows {
        grove.push(row?);
    }

    Ok(grove)
}

#[test]
fn fifth_column_of_sample() {
    let column = vec![3, 2, 2, 9, 0];
    assert_eq!(
        look(column.iter()),
        vec![0, 3]
    )
}

#[test]
fn second_column_of_sample() {
    let column = vec![0, 5, 5, 3, 5];
    assert_eq!(
        look(column.iter()),
        vec![0, 1]
    )
}

#[test]
fn all_visible() {
    let column = vec![0, 1, 4, 6];
    assert_eq!(
        look(column.iter()),
        vec![0, 1, 2, 3]
    )
}

#[test]
fn one_visible() {
    let column = vec![0, 1, 4, 6];
    assert_eq!(
        look(column.iter().rev()),
        vec![0]
    )
}

#[test]
fn scenic_perch() {
    let grove = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    assert_eq!(scenic_score(&grove, (3, 2)), 2 * 2 * 1 * 2);
}
