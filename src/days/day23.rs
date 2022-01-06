use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day23();

impl AOCDay for Day23 {
    fn part1(&self, _input: &str) -> Option<String> { None }
    fn part2(&self, _input: &str) -> Option<String> { None }
    fn get_num(&self) -> u32 { 23 }
}

pub fn get() -> Day23 {Day23()}
