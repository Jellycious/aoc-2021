use crate::AOCDay;

use std::str::FromStr;

/*
 * Template for a implementing a day
 */

pub struct Day1();

impl AOCDay for Day1 {
    fn part1(&self, _input: &str) -> Option<String> { Some(increased_measurements(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(sliding_window(_input)) }
    fn get_num(&self) -> u32 { 1 }
}

fn increased_measurements(input: &str) -> String {
    let mut measurements = input.split_whitespace();
    let mut prev = parse_u32(measurements.next().unwrap());
    let mut count: u32 = 0;

    for num_str in measurements {
        let num = parse_u32(num_str);
        if num > prev {
            count+=1;
        }
        prev = num;
    }
    count.to_string()
}

fn sliding_window(input: &str) -> String {
    let measurements: Vec<u32> = input.split_whitespace().map(|s| parse_u32(s)).collect();
    let len = measurements.len();

    let mut count = 0;
    let mut prev_sum = measurements[0] + measurements[1] + measurements[2];
    for i in 1..(len-2) {
        let sum = measurements[i] + measurements[i+1] + measurements[i+2];
        if sum > prev_sum { count += 1;}
        prev_sum = sum;
    }
    count.to_string()
}

fn parse_u32(num: &str) -> u32 {
    u32::from_str(num).unwrap()
}

pub fn get() -> Day1 {Day1()}

