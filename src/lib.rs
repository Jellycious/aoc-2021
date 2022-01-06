/*
 * Based on input get the correct day and part
 *
 */
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

mod days;
mod web;
mod utils;

/// Part enum for puzzle part 1 or part 2
pub enum Part {
    One,
    Two
}

impl From<&Part> for u32 {
    fn from(item: &Part) -> u32 {
        match item {
            Part::One => 1,
            Part::Two => 2,
        }
    }
}

/// AOCDay trait
pub trait AOCDay {
    fn part1(&self, input: &str) -> Option<String>;
    fn part2(&self, input: &str) -> Option<String>;
    fn get_num(&self) -> u32;
}


pub fn get_day_header(day: u32) -> String {
    let filepath = utils::get_description_filepath(day);
    if filepath.is_err() {
        return web::get_day_header(day).unwrap();
    }
    let filepath = filepath.unwrap();
    if filepath.exists() {
        // retrieve from file
        let contents = fs::read_to_string(filepath);
        if contents.is_err() {
            panic!("Could not read description file");
        }
        String::from(contents.unwrap().lines().next().unwrap())
    }else {
        let desc = get_day_description(day);
        String::from(desc.lines().next().unwrap())
    }
}

/// Gets input from local file or aternatively from the web and stores it in local file.
pub fn get_day_input(num: u32) -> String  {
    let filepath = utils::get_input_filepath(num);
    if filepath.is_err() {
        // get file from web instead
        return web::get_day_input(num);
    }
    let filepath = filepath.unwrap();

    if filepath.exists() {
        // return contents
        let contents = fs::read_to_string(filepath);
        if contents.is_err() {
            panic!("Could not read input file");
        }
        contents.unwrap()
    }else {
        // get input from web and store in file
        let input = web::get_day_input(num);
        let file = File::create(&filepath);
        if file.is_err() {
            eprintln!("{:?}", filepath);
            eprintln!("{}", file.unwrap_err());
            eprintln!("Could not store input");
        }else {
            // write to input file
            let res = file.unwrap().write(input.as_bytes());
            if res.is_err() {
                eprintln!("{:?}", filepath);
                eprintln!("{}", res.unwrap_err());
                eprintln!("Could not write to input file");
            }
        }
        input
    }
}

/// Gets description from web and stores it in local file.
pub fn get_day_description(num: u32) -> String {
    let filepath = utils::get_description_filepath(num);
    if filepath.is_err() {
        // get it from web
        return web::get_description(num).unwrap();
    }
    let filepath = filepath.unwrap();
    // get input from web and store in file
    let input = web::get_description(num).unwrap();
    let file = File::create(&filepath);
    if file.is_err() {
        eprintln!("{:?}", filepath);
        eprintln!("{}", file.unwrap_err());
        eprintln!("Could not store description");
    }else {
        // write to input file
        let res = file.as_ref().unwrap().write(input.as_bytes());
        if res.is_err() {
            eprintln!("{:?}", filepath);
            eprintln!("{}", res.unwrap_err());
            eprintln!("Could not write to description file");
        }
    }
    return input;
}

/// Solves a specific day.
pub fn solve(num: u32, part: Part) -> Option<(String, Duration)> {
    let day = day(num);
    let input = get_day_input(num);

    match part {
        Part::One => {
            let now = Instant::now();
            let sol = day.part1(&input);
            let dur = now.elapsed();
            if sol.is_some() {
                return Some((sol.unwrap(), dur));
            }
            None
        },
        Part::Two => {
            let now = Instant::now();
            let sol = day.part2(&input);
            let dur = now.elapsed();
            if sol.is_some() {
                return Some((sol.unwrap(), dur));
            }
            None
        }
    }
}

fn day(num:u32) -> Box<dyn AOCDay> {
    match num {
        1 => Box::new(days::day1::get()),
        2 => Box::new(days::day2::get()),
        3 => Box::new(days::day3::get()),
        4 => Box::new(days::day4::get()),
        5 => Box::new(days::day5::get()),
        6 => Box::new(days::day6::get()),
        7 => Box::new(days::day7::get()),
        8 => Box::new(days::day8::get()),
        9 => Box::new(days::day9::get()),
        10 => Box::new(days::day10::get()),
        11 => Box::new(days::day11::get()),
        12 => Box::new(days::day12::get()),
        13 => Box::new(days::day13::get()),
        14 => Box::new(days::day14::get()),
        15 => Box::new(days::day15::get()),
        16 => Box::new(days::day16::get()),
        17 => Box::new(days::day17::get()),
        18 => Box::new(days::day18::get()),
        19 => Box::new(days::day19::get()),
        20 => Box::new(days::day20::get()),
        21 => Box::new(days::day21::get()),
        22 => Box::new(days::day22::get()),
        23 => Box::new(days::day23::get()),
        24 => Box::new(days::day24::get()),
        25 => Box::new(days::day25::get()),
        _ => panic!("Day does not exist"),
    }
}


pub fn run() {
    solve(1, Part::One);
}
