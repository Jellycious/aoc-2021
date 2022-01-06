use crate::AOCDay;

/*
 * Template for a implementing a day
 */

type Fishes = Vec<u64>;

pub struct Day6();

impl AOCDay for Day6 {
    fn part1(&self, input: &str) -> Option<String> { Some(part1(input)) }
    fn part2(&self, input: &str) -> Option<String> { Some(part2(input)) }
    fn get_num(&self) -> u32 { 6 }
}

pub fn get() -> Day6 {Day6()}

fn part1(input: &str) -> String {
    let mut fishes = parse(&input);
    simulate_days(80, &mut fishes);
    let count: u64 = fishes.iter().sum();
    String::from(format!("{}", count))
}

fn part2(input: &str) -> String {
    let mut fishes = parse(&input);
    simulate_days(256, &mut fishes);
    let count: u64 = fishes.iter().sum();
    String::from(format!("{}", count))
}

fn simulate_days(days: u64, fishes: &mut Fishes) {
    for _ in 0..days {
        simulate_day(fishes);
    }
}

fn simulate_day(fishes: &mut Fishes) {
    let fish_old = fishes.clone();
    // update fishes
    for i in 0..8 {
        // older fish become younger
        fishes[i] = fish_old[i+1]
    }
    fishes[8] = fish_old[0]; // new fish are born
    fishes[6] += fish_old[0];
}

fn test_input() -> String {
    String::from("3,4,3,1,2")
}

fn parse(input: &str) -> Fishes {
    let nums: Vec<&str> = input.trim_end().split(',').collect(); 
    let mut ages: Vec<u64> = Vec::new();
    for n in nums {
        ages.push(n.parse().unwrap());
    }

    let mut fishes: Vec<u64> = Vec::new();
    for _ in 0..9 {
        fishes.push(0);
    }

    for age in ages {
        fishes[age as usize] += 1;
    }
    fishes
}
