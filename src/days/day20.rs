use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day20();

impl AOCDay for Day20 {
    fn part1(&self, _input: &str) -> Option<String> { None }
    fn part2(&self, _input: &str) -> Option<String> { None }
    fn get_num(&self) -> u32 { 20 }
}

pub fn get() -> Day20 {Day20()}
