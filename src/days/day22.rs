use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day22();

impl AOCDay for Day22 {
    fn part1(&self, _input: &str) -> Option<String> { None }
    fn part2(&self, _input: &str) -> Option<String> { None }
    fn get_num(&self) -> u32 { 22 }
}

pub fn get() -> Day22 {Day22()}
