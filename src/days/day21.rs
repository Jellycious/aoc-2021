use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day21();

impl AOCDay for Day21 {
    fn part1(&self, _input: &str) -> Option<String> { None }
    fn part2(&self, _input: &str) -> Option<String> { None }
    fn get_num(&self) -> u32 { 21 }
}

pub fn get() -> Day21 {Day21()}
