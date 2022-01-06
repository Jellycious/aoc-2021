use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day25();

impl AOCDay for Day25 {
    fn part1(&self, _input: &str) -> Option<String> { None }
    fn part2(&self, _input: &str) -> Option<String> { None }
    fn get_num(&self) -> u32 { 25 }
}

pub fn get() -> Day25 {Day25()}
