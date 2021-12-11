fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: cargo run -- puzzle-input");
        std::process::exit(1);
    }

    let puzzle = std::fs::read_to_string(&args[1]).unwrap();
    let sensors = make_sensors(&puzzle).unwrap();
    let (Coord(min_x, max_x), Coord(min_y, max_y)) = tunnel_bounds(&sensors).unwrap();
    println!("tunnels span x={}..={} and y={}..={}", min_x, max_y, min_y, max_y);

    // let y_of_interest = 10;
    let y_of_interest = 2000000;
    let mut in_radius = 0;

    for x in min_x..=max_x {
        let not_sensor = !sensors.iter().map(|sensor| sensor.sensor == Coord(x, y_of_interest)).any(|found| found);
        let not_beacon = !sensors.iter().map(|sensor| sensor.beacon == Coord(x, y_of_interest)).any(|found| found);
        if not_sensor && not_beacon && in_sensor_radius(&sensors, Coord(x, y_of_interest)) {
            in_radius += 1;
        }
    }

    let in_cells_of_interest = max_x - min_x + 1;
    println!("part one: {} / {}", in_radius, in_cells_of_interest);

    // println!("part two: {:?}", tuning_frequency(&sensors, (0, 20)));
    println!("part two: {:?}", tuning_frequency(&sensors, (0, 4000000)).map(|Coord(x, y)| 4000000 * x + y));

    // _display_sensors(&sensors);
}

fn tuning_frequency(sensors: &[Sensor], bounds: (isize, isize)) -> Option<Coord> {
    // > There is never a tie where two beacons are the same distance to a sensor.

    for sensor in sensors {
        let north = sensor.sensor + Coord(0, sensor.radius);
        let east = sensor.sensor + Coord(sensor.radius, 0);
        let south = sensor.sensor + Coord(0, -sensor.radius);
        let west = sensor.sensor + Coord(-sensor.radius, 0);

        let mut perimeter = north + Coord(0, 1);
        while perimeter != east + Coord(1, 0) {
            perimeter = perimeter + Coord(1, -1);
            let in_region =
                bounds.0 <= perimeter.0 && perimeter.0 <= bounds.1 &&
                bounds.0 <= perimeter.1 && perimeter.1 <= bounds.1;
            if in_region && !in_sensor_radius(sensors, perimeter) {
                return Some(perimeter);
            }
        }

        while perimeter != south + Coord(0, -1) {
            perimeter = perimeter + Coord(-1, -1);
            let in_region =
                bounds.0 <= perimeter.0 && perimeter.0 <= bounds.1 &&
                bounds.0 <= perimeter.1 && perimeter.1 <= bounds.1;
            if in_region && !in_sensor_radius(sensors, perimeter) {
                return Some(perimeter);
            }
        }

        while perimeter != west + Coord(-1, 0) {
            perimeter = perimeter + Coord(-1, 1);
            let in_region =
                bounds.0 <= perimeter.0 && perimeter.0 <= bounds.1 &&
                bounds.0 <= perimeter.1 && perimeter.1 <= bounds.1;
            if in_region && !in_sensor_radius(sensors, perimeter) {
                return Some(perimeter);
            }
        }

        while perimeter != north + Coord(0, 1) {
            perimeter = perimeter + Coord(1, 1);
            let in_region =
                bounds.0 <= perimeter.0 && perimeter.0 <= bounds.1 &&
                bounds.0 <= perimeter.1 && perimeter.1 <= bounds.1;
            if in_region && !in_sensor_radius(sensors, perimeter) {
                return Some(perimeter);
            }
        }
    }

    None
}

fn _display_sensors(sensors: &[Sensor]) {
    let (Coord(min_x, max_x), Coord(min_y, max_y)) = tunnel_bounds(&sensors).unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let here = Coord(x, y);
            let sensor_here = sensors.iter().map(|sensor| sensor.sensor == here).any(|found| found);
            let beacon_here = sensors.iter().map(|sensor| sensor.beacon == here).any(|found| found);
            if sensor_here {
                print!("S");
            } else if beacon_here {
                print!("B");
            } else if in_sensor_radius(sensors, here) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Coord(isize, isize);

impl std::ops::Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Neg for Coord {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Coord(-self.0, -self.1)
    }
}

impl Coord {
    fn mag(&self) -> isize {
        self.0.abs() + self.1.abs()
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

#[derive(Debug)]
struct Sensor {
    sensor: Coord,
    beacon: Coord,
    radius: isize,
}

fn tunnel_bounds(sensors: &[Sensor]) -> Option<(Coord, Coord)> {
    let beacons = sensors.iter().map(|sensor| sensor.beacon);
    let sensors = sensors.iter().map(|sensor| sensor.sensor);

    let sensor_min_x = sensors.clone().map(|sensor| sensor.0).min()?;
    let beacon_min_x = beacons.clone().map(|beacon| beacon.0).min()?;
    let min_x = std::cmp::min(sensor_min_x, beacon_min_x);

    let sensor_max_x = sensors.clone().map(|sensor| sensor.0).max()?;
    let beacon_max_x = beacons.clone().map(|beacon| beacon.0).max()?;
    let max_x = std::cmp::max(sensor_max_x, beacon_max_x);

    let sensor_min_y = sensors.clone().map(|sensor| sensor.1).min()?;
    let beacon_min_y = beacons.clone().map(|beacon| beacon.1).min()?;
    let min_y = std::cmp::min(sensor_min_y, beacon_min_y);

    let sensor_max_y = sensors.clone().map(|sensor| sensor.1).max()?;
    let beacon_max_y = beacons.clone().map(|beacon| beacon.1).max()?;
    let max_y = std::cmp::max(sensor_max_y, beacon_max_y);

    return Some((Coord(min_x, max_x), Coord(min_y, max_y)))
}

fn in_sensor_radius(sensors: &[Sensor], coord: Coord) -> bool {
    sensors.iter()
        .map(|sensor| (coord - sensor.sensor).mag() <= sensor.radius)
        .any(|in_radius| in_radius)
}

fn make_sensors(lines: &str) -> Option<Vec<Sensor>> {
    let mut sensors = Vec::new();

    // TODO: How to fold a Vec<Option<Coord>> into an Option<Vec<Coord>>?
    let beacons: Vec<Coord> = lines.lines()
        .flat_map(|line| Some(parse_sensor(line)?.1))
        .collect();

    for line in lines.lines() {
        let (sensor, _) = parse_sensor(line)?;
        let (radius, beacon) = closest_beacon_to(&beacons, sensor)?;
        sensors.push(Sensor { sensor, beacon, radius });
    }

    Some(sensors)
}

fn parse_sensor(line: &str) -> Option<(Coord, Coord)> {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let mut sensor_and_beacon = line.split(": ");
    let sensor = parse_coordinate(sensor_and_beacon.next()?.strip_prefix("Sensor at ")?)?;
    let beacon = parse_coordinate(sensor_and_beacon.next()?.strip_prefix("closest beacon is at ")?)?;
    Some((sensor, beacon))
}

fn parse_coordinate(text: &str) -> Option<Coord> {
    // x=-2, y=15
    let mut x_and_y = text.split(", ");
    let x = x_and_y.next()?.strip_prefix("x=")?.parse::<isize>().ok()?;
    let y = x_and_y.next()?.strip_prefix("y=")?.parse::<isize>().ok()?;
    Some(Coord(x, y))
}

fn closest_beacon_to(beacons: &[Coord], sensor: Coord) -> Option<(isize, Coord)> {
    beacons.iter()
        .map(|&beacon| ((beacon - sensor).mag(), beacon))
        .min_by_key(|pair| pair.0)
}

#[test]
fn parse_negative_positive() {
    assert_eq!(parse_coordinate("x=-2, y=15"), Some(Coord(-2, 15)));
}

#[test]
fn parse_first_line_of_sample() {
    assert_eq!(
        parse_sensor("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
        Some((Coord(2, 18), Coord(-2, 15)))
    );
}

#[test]
fn closest_beacon() {
    assert_eq!(
        closest_beacon_to(
            &[(7, 0), (1, 1), (8, 3), (1, 5), (5, 6)].map(|(x, y)| Coord(x, y)),
            Coord(4, 3)
        ),
        Some((4, Coord(8, 3)))
    );
}
