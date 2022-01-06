use crate::AOCDay;

/*
 * Day 17: Trick Shot
 *
 * Find velocities such that the probe is whithin an area at a certain point in time.
 */

pub struct Day17();

impl AOCDay for Day17 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 17 }
}

pub fn get() -> Day17 {Day17()}

#[derive(Debug)]
struct Area {
    xlim: (i32, i32), // (xmin, xmax)
    ylim: (i32, i32), // (ymin, ymax)
}

#[derive(Debug)]
struct Probe {
    velocity: (i32, i32),
    position: (i32, i32)
}

#[derive(Debug)]
enum RelPos {
    Right,
    Down,
    RightDown,
}


fn part1(input: &str) -> String {
    let area = parse(&input);
    let (_,y) = highest_trajectory(&area).unwrap();
    let n = (y * (y+1)) / 2;
    n.to_string()
}

fn part2(input: &str) -> String {
    let area = parse(&input);
    let n = number_of_hits(&area);
    n.to_string()
}

fn highest_trajectory(area: &Area) -> Option<(i32, i32)> {
    let y_max = max_velocity_y(&area);
    let x_start = min_velocity_x(&area);
    let x_end = max_velocity_x(&area);
    for y in (0..y_max+1).into_iter().rev() {
        for x in x_start..x_end+1 {
            if simulate(x, y, &area) {
                return Some((x,y));
            };
        }
    }
    None
}

fn number_of_hits(area: &Area) -> u32 {
    let y_max = max_velocity_y(&area);
    let y_min = min_velocity_y(&area);
    let x_min = min_velocity_x(&area);
    let x_max = max_velocity_x(&area);

    let mut count = 0; // O(n^2)
    for y in y_min..y_max+1 {
        for x in x_min..x_max+1 {
            if simulate(x, y, &area) { 
                count += 1;
            }; 
        }
    }
    count
}

fn simulate_step(probe: &mut Probe) {
    probe.position = (probe.position.0 + probe.velocity.0, probe.position.1 + probe.velocity.1); // update position
    let (x_v, y_v) = probe.velocity;
    let dx = if x_v == 0 {0} else {-x_v / x_v};
    probe.velocity = (x_v + dx, y_v - 1);
}

fn simulate(x_v: i32, y_v: i32, target: &Area) -> bool {
    let mut p = Probe {velocity: (x_v, y_v), position: (0, 0)};
    while !past_area(p.position.0, p.position.1, target).0 {
        if within_area(p.position.0, p.position.1, target) {return true;}
        simulate_step(&mut p);
    }
    return false;
}

fn within_area(x: i32, y: i32, area: &Area) -> bool {x >= area.xlim.0 && x <= area.xlim.1 && y >= area.ylim.0 && y <= area.ylim.1}

fn past_area(x: i32, y: i32, area: &Area) -> (bool, RelPos) {
    let x_past = x > area.xlim.1;
    let y_past = y < area.ylim.0;
    match (x_past, y_past) {
        (true, true) => {(true, RelPos::RightDown)},
        (true, false) => {(true, RelPos::Right)},
        (false, true) => {(true, RelPos::Down)},
        (false, false) => {(false, RelPos::RightDown)},
    }
}

fn min_velocity_x(area: &Area) -> i32 {
    let mut i = 0;
    loop {
        let n = (i * (i+1)) / 2;
        if n >= area.xlim.0 {return i;}
        i += 1;
    }
}
fn max_velocity_x(area: &Area) -> i32 { area.xlim.1 }

fn min_velocity_y(area: &Area) -> i32 { area.ylim.0 }
fn max_velocity_y(area: &Area) -> i32 { i32::abs(area.ylim.0) }



fn parse(input: &str) -> Area {
    let input = &input[13..].trim_end();
    let mut s = input.split(", ");
    let x_s = &s.next().unwrap()[2..]; 
    let y_s = &s.next().unwrap()[2..]; 

    let mut x_nums = x_s.split("..");
    let x1 = i32::from_str_radix(x_nums.next().unwrap(), 10).unwrap();
    let x2 = i32::from_str_radix(x_nums.next().unwrap(), 10).unwrap();
    let (xmin, xmax) = (i32::min(x1,x2), i32::max(x1,x2));

    let mut y_nums = y_s.split("..");
    let y1 = i32::from_str_radix(y_nums.next().unwrap(), 10).unwrap();
    let y2 = i32::from_str_radix(y_nums.next().unwrap(), 10).unwrap();
    let (ymin, ymax) = (i32::min(y1,y2), i32::max(y1,y2));
    Area {xlim: (xmin,xmax), ylim: (ymin,ymax)}
}

fn test_input() -> String {
    String::from("target area: x=20..30, y=-10..-5")
}
