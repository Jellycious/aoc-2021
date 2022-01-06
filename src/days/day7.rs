use crate::AOCDay;

// [1] https://en.wikipedia.org/wiki/Triangular_number
// Tried a more functional approach this time.

pub struct Day7();

impl AOCDay for Day7 {
    fn part1(&self, input: &str) -> Option<String> { Some(String::from(format!("{}", cheapest_alignment_dist(input)))) }
    fn part2(&self, input: &str) -> Option<String> { Some(String::from(format!("{}", cheapest_alignment_fuel(input)))) }
    fn get_num(&self) -> u32 { 7 }
}

pub fn get() -> Day7 {Day7()}

fn cheapest_alignment_dist(input: &str) -> u32{
    let mut crabs = parsing(&input);
    crabs.sort();
    calculate_dist(crabs[crabs.len()/2], &crabs)
}

fn cheapest_alignment_fuel(input: &str) -> u32{
    let crabs = parsing(&input);
    let mean: u32 = crabs.iter().sum::<u32>() as u32 / crabs.len() as u32; // answer is around mean
    ((mean-1)..).take(3).map(|i| calculate_fuel(i, &crabs)).min().unwrap()
}

fn calculate_fuel(x: u32, crabs: &Vec<u32>) -> u32 {
    crabs.iter().map(|c| ((dist(x,*c)+1)*dist(x,*c)) / 2).sum::<u32>() // utilize [1]
}

fn calculate_dist(x: u32, crabs: &Vec<u32>) -> u32 {
    crabs.iter().map(|c| dist(x, *c)).sum::<u32>()
}

fn test_input() -> String {
    String::from("16,1,2,0,4,2,7,1,2,14")
}

fn dist(x: u32, y: u32) -> u32 {
    i32::abs(x as i32 - y as i32) as u32
}

fn parsing(input: &str) -> Vec<u32> {
    let nums = input.trim_end().split(',');
    let mut v = Vec::new();
    for n in nums {
        v.push(n.parse().unwrap());
    }
    v
}

