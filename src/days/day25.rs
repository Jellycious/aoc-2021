use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day25();

impl AOCDay for Day25 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(String::from("🌟 Happy Christmas!!!🎅")) }
    fn get_num(&self) -> u32 { 25 }
}

pub fn get() -> Day25 {Day25()}

// --- STRUCTURES ---
#[derive(Debug,Copy,Clone,PartialEq,Eq)]
enum S {
    Empty,
    South,
    East,
}

fn part1(input: &str) -> String {
    let mut sea = parse(&input);
    let mut i = 0;
    while iterate(&mut sea) {
        i+=1;
    }
    format!("{}", i+1)
}

fn iterate(sea: &mut Vec<Vec<S>>) -> bool {
    let mut updated_sea = sea.clone();
    // First East Seacucumbers move
    let mut updated = false;
    for y in 0..sea.len() {
        for x in 0..sea[0].len()-1 {
            if sea[y][x] == S::East && sea[y][x+1] == S::Empty{
                updated_sea[y][x] = S::Empty;
                updated_sea[y][x+1] = S::East;
                updated = true;
            }
        }
        if sea[y][sea[0].len()-1] == S::East && sea[y][0] == S::Empty { // handle edge case
                updated_sea[y][sea[0].len()-1] = S::Empty;
                updated_sea[y][0] = S::East;
                updated = true;
        }
    }
    *sea = updated_sea;
    updated_sea = sea.clone();
    // Second South Seacucumbers move
    for y in 0..sea.len()-1 {
        for x in 0..sea[0].len() {
            if sea[y][x] == S::South && sea[y+1][x] == S::Empty{
                updated_sea[y][x] = S::Empty;
                updated_sea[y+1][x] = S::South;
                updated = true;
            }
        }
    }
    for x in 0..sea[0].len() { // handle edge case
        if sea[sea.len()-1][x] == S::South && sea[0][x] == S::Empty{
            updated_sea[sea.len()-1][x] = S::Empty;
            updated_sea[0][x] = S::South;
            updated = true;
        }
    }
    *sea = updated_sea;
    updated
}

// --- TEST INPUTS ---
fn test_input() -> String {
    String::from("v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>")
}

// --- PARSING ---
fn parse(input: &str) -> Vec<Vec<S>> {
    let mut grid = Vec::new();
    for line in input.lines().into_iter() {
        let chars = line.trim_end().chars();
        let mut v = Vec::new();
        for c in chars {
            match c {
                'v' => {v.push(S::South)},
                '>' => {v.push(S::East)},
                '.' => {v.push(S::Empty)},
                _ => {panic!()},
            }
        }
        grid.push(v);
    }
    grid
}
