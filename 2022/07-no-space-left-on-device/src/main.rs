use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::rc::{Rc, Weak};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        return Err(Box::from("usage: cargo run -- puzzle-input"));
    }

    let commands = read_puzzle(&args[1])?;
    let filesystem = make_filesystem(commands.into_iter());

    // println!("{}", filesystem._show());
    // println!("");

    println!("part one: {}", part_one(filesystem.clone()));
    println!("part two: {}", part_two(filesystem.clone())
        .expect("expect some directory to meet threshold"));

    Ok(())
}

fn part_one(dir: Rc<Directory>) -> u32 {
    // Very inefficient because `size` recurs.
    let size = dir.size();
    let capped = if size <= 100_000 { size } else { 0 };

    capped + dir.contents.borrow()
            .values()
            .filter_map(|file| file.to_directory())
            .map(|file| part_one(file))
            .sum::<u32>()
}

fn part_two(dir: Rc<Directory>) -> Option<u32> {
    let disk_size = 70_000_000;
    let update_size = 30_000_000;
    let free_space = disk_size - dir.size();
    let required_space = update_size - free_space;

    fn go(dir: Rc<Directory>, threshold: u32) -> Option<u32> {
        let child_candidate = dir.contents.borrow()
            .values()
            .filter_map(|file| file.to_directory())
            .filter_map(|file| go(file, threshold))
            .min();

        // Very inefficient because `size` recurs.
        let size = dir.size();

        // The parent directory must be at least as large as the smallest child that meets the
        // threshold, so we may safely discard it.
        child_candidate.or(
            if size >= threshold {
                Some(size)
            } else {
                None
            }
        )
    }

    go(dir, required_space)
}

#[derive(Debug)]
enum Command {
    Change(Change),
    List(Vec<ListLine>)
}

#[derive(Debug)]
enum Change {
    In(String), // cd aoc
    Out,        // cd ..
    Root        // cd /
}

#[derive(Debug)]
struct ListLine (String, Option<u32>);

enum File {
    Directory(Rc<Directory>),
    Terminal(u32),
}

struct Directory {
    name: String,
    up: Option<Weak<Directory>>,
    contents: RefCell<HashMap<String, File>>
}

impl File {
    fn size(&self) -> u32 {
        match self {
            File::Directory(dir) => dir.size(),
            File::Terminal(size) => *size,
        }
    }

    fn to_directory(&self) -> Option<Rc<Directory>> {
        match self {
            File::Directory(dir) => Some(dir.clone()),
            File::Terminal(_) => None,
        }
    }
}

impl Directory {
    fn _show(&self) -> String {
        fn go(dir: &Directory, depth: usize) -> String {
            let mut readable = Vec::new();
            let contents: &HashMap<String, File> = &dir.contents.borrow();
            for (name, file) in contents {
                match file {
                    File::Directory(dir) => {
                        readable.push(format!("{}- {} (dir)", "  ".repeat(depth), name));
                        readable.push(go(dir, depth + 1));
                    },
                    File::Terminal(size) => {
                        readable.push(format!("{}- {} (file, size={})", "  ".repeat(depth), name, size));
                    }
                }
            }

            if readable.len() < 1 {
                readable.push(format!("{}- *empty*", "  ".repeat(depth)));
            }

            readable.join("\n")
        }

        ["- / (dir)".to_string(), go(self, 1)].join("\n")
    }

    fn open(&self, name: &str) -> Option<Rc<Directory>> {
        match self.contents.borrow().get(name) {
            Some(File::Directory(found)) => Some(found.clone()),
            _ => None,
        }
    }

    fn size(&self) -> u32 {
        self.contents.borrow().values()
            .map(|file| file.size())
            .sum::<u32>()
    }
}

fn make_filesystem(commands: impl Iterator<Item=Command>) -> Rc<Directory> {
    let root = Rc::new(Directory {
        name: "/".to_string(),
        up: None,
        contents: RefCell::new(HashMap::new()),
    });

    let mut working = root.clone();

    for command in commands {
        match command {
            Command::Change(change) => {
                working = match change {
                    Change::In(dir) =>
                        working.open(&dir)
                            .expect(format!("cannot `cd {}` from `{}`", dir, working.name).as_str()),
                    Change::Out =>
                        working.up.as_ref()
                            .expect(format!("cannot `cd ..` from `{} when at `/``", working.name).as_str())
                            .upgrade()
                            .expect(format!("cannot `cd ..` from `{}` because dropped parent", working.name).as_str()),
                    Change::Root =>
                        root.clone(),
                };
            },
            Command::List(lines) => {
                for line in lines {
                    let mut files = working.contents.borrow_mut();

                    // `HashMap::insert` destroys the conflicting file if it exists in the directory.
                    files.insert(line.0.clone(), match line.1 {
                        None => {
                            File::Directory(Rc::new(Directory {
                                name: line.0,
                                up: Some(Rc::downgrade(&working)),
                                contents: RefCell::new(HashMap::new()),
                            }))
                        },
                        Some(size) =>
                            File::Terminal(size),
                    });
                }
            }
        }
    }

    root
}

fn read_puzzle(puzzle: &str) -> Result<Vec<Command>, Box<dyn Error>> {
    let puzzle = fs::read_to_string(puzzle)?;
    let mut commands = Vec::new();

    for output in puzzle.split('$').skip(1) {
        let mut output = output.lines();

        let mut command = output.next()
            .ok_or(Box::<dyn Error>::from("expect command"))?
            .split_ascii_whitespace();

        let program = command.next()
            .ok_or(Box::<dyn Error>::from("expect program name"))?;

        match program {
            "cd" => {
                let argument = command.next()
                    .ok_or(Box::<dyn Error>::from("expect argument to `cd`"))?;

                commands.push(Command::Change(match argument {
                    ".." => Change::Out,
                    "/" => Change::Root,
                    dir => Change::In(dir.to_string()),
                }));
            },
            "ls" => {
                let mut inflated = Vec::new();

                for line in output {
                    let mut stats = line.split_ascii_whitespace();

                    let first_column = stats.next()
                        .ok_or(Box::<dyn Error>::from("expect first column in `ls` output"))?;

                    let name = stats.next()
                        .ok_or(Box::<dyn Error>::from(format!("expect file in `ls` output; first column is '{}'", first_column)))?
                        .to_string();

                    let size = match first_column {
                        "dir" => None,
                        size => Some(size.parse::<u32>()?),
                    };

                    inflated.push(ListLine (name, size))
                }

                commands.push(Command::List(inflated));
            },
            bin => {
                return Err(Box::<dyn Error>::from(format!("unexpected program `{}`", bin)));
            }
        }
    }

    Ok(commands)
}
