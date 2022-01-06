use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day3();

impl AOCDay for Day3 {
    fn part1(&self, input: &str) -> Option<String> { Some(power_consumption(input)) }
    fn part2(&self, input: &str) -> Option<String> { Some(life_support_rating(input)) }
    fn get_num(&self) -> u32 { 3 }
}

pub fn get() -> Day3 {Day3()}

fn get_test_input() -> String {
    String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010")
}

fn power_consumption(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let number_count = lines.len() / 2;
    let number_size = lines[0].len();
    let ones = count_ones(&lines);

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for i in 0..number_size {
        if ones[i] > number_count {
            // 1 heavy
            gamma = (gamma << 1) | 0x01;
            epsilon = epsilon << 1;
        }else {
            // 0 heavy
            gamma = gamma << 1;
            epsilon = epsilon << 1 | 0x01;
        }
    }
    String::from(format!("{}", gamma * epsilon).as_str())
}

fn life_support_rating(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let mut oxygen_candidates = lines.clone();
    let mut co2_candidates = lines;

    for i in 0..oxygen_candidates.len() {
        let number_count = (oxygen_candidates.len() as f32) / 2.0;
        let ones = count_ones(&oxygen_candidates);
        if ones[i] as f32 >= number_count {
            oxygen_candidates = eliminate_with_char_at_index(i, '0', oxygen_candidates);
        }else {
            oxygen_candidates = eliminate_with_char_at_index(i, '1', oxygen_candidates);
        }

        if oxygen_candidates.len() <= 1 {break;}
    }

    for i in 0..co2_candidates.len() {
        let number_count = co2_candidates.len() as f32 / 2.0;
        let ones = count_ones(&co2_candidates);
        if ones[i] as f32 >= number_count {
            // '0' least or equally common
            co2_candidates = eliminate_with_char_at_index(i, '1', co2_candidates);
        }else {
            co2_candidates = eliminate_with_char_at_index(i, '0', co2_candidates);
        }

        if co2_candidates.len() <= 1 {break;}
    }


    
    let oxygen_rating = u32::from_str_radix(oxygen_candidates.pop().unwrap(), 2).unwrap();
    let co2_rating = u32::from_str_radix(co2_candidates.pop().unwrap(), 2).unwrap();
     
    String::from(format!("{}", oxygen_rating * co2_rating))
}

/// Removes string with a character at an index
fn eliminate_with_char_at_index(index: usize, c: char, vec: Vec<&str>) -> Vec<&str> {
    let mut result = Vec::new();
    for e in vec {
        let chars: Vec<char> = e.chars().collect(); 
        if chars[index] != c {
            result.push(e);
        }
    }
    result
}

fn count_ones(v: &Vec<&str>) -> Vec<usize> {
    let num_len = v[0].len();
    let mut vec = Vec::with_capacity(num_len);
    for _ in 0..num_len { vec.push(0);} // fill vector with zeroes
    for l in v {
        let n: u32 = u32::from_str_radix(l, 2).unwrap();
        for i in 0..num_len {
            let s = 0x01 & (n >> i);
            if s > 0 { vec[num_len-1-i] += 1; }
        }
    }
    vec 
}

