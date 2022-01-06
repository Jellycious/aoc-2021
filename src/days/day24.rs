use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day24();

impl AOCDay for Day24 {
    fn part1(&self, _input: &str) -> Option<String> { None }
    fn part2(&self, _input: &str) -> Option<String> { None }
    fn get_num(&self) -> u32 { 24 }
}

pub fn get() -> Day24 {Day24()}
