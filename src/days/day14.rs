use crate::AOCDay;

use std::collections::HashMap;

/*
 * Template for a implementing a day
 */

pub struct Day14();

impl AOCDay for Day14 {
    fn part1(&self, _input: &str) -> Option<String> { Some(solve(_input, 10)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(solve(_input, 40)) }
    fn get_num(&self) -> u32 { 14 }
}

pub fn get() -> Day14 {Day14()}

type RuleSet = HashMap<(char, char), char>;

fn solve(input: &str, steps: usize) -> String {
    let (template, ruleset) = parsing(&input);
    let last_char = template.chars().rev().next().unwrap();
    let pair_counts = process_polymer(&template, &ruleset, steps);
    let char_list = char_list(&ruleset);
    let (min, max) = compute_char_occurence(&pair_counts, &char_list, last_char);
    String::from(format!("{}", max - min))
}

fn char_list(ruleset: &RuleSet) -> Vec<char> { // A vector containing all possible chars in the polymer chain
    let mut v: Vec<char> = Vec::new();
    for (c1,c2) in ruleset.keys() {
        v.push(*c1);
        v.push(*c2);
    }
    v.sort();
    v.dedup();
    v
}

fn process_polymer(template: &str, ruleset: &RuleSet, steps: usize) -> HashMap<(char,char), u64> {
    /*
     * Processes the polymer
     * Keeps track of the number of pairs present after updating the polymer chain `steps` times.
     */
    let mut pair_counts: HashMap<(char, char), u64> = HashMap::new();

    // create pair counts
    for (c1, c2) in ruleset.keys() {
        pair_counts.insert((*c1,*c2), 0);
    }
    let mut chars = template.chars();
    let mut c1 = chars.next().unwrap();
    while let Some(c2) = chars.next() {
        let cur = *pair_counts.get(&(c1,c2)).unwrap();
        pair_counts.insert((c1,c2), cur + 1);
        c1 = c2;
    }

    for _ in 0..steps {
        // update pair counts according to ruleset
        let mut new_pair_counts: HashMap<(char,char), u64> = HashMap::new();

        for (c1, c2) in pair_counts.keys() {
            let c = ruleset.get(&(*c1,*c2)).unwrap();
            let p1 = (*c1, *c); // newly created pair
            let p2 = (*c, *c2); // newly created pair

            let count = pair_counts.get(&(*c1,*c2)).unwrap();

            let cp1 = new_pair_counts.get_mut(&p1); // &mut u64
            if cp1.is_some() {
                cp1.map(|s| *s = *s + count);
            }else {
                new_pair_counts.insert(p1, *count);
            }

            let cp2 = new_pair_counts.get_mut(&p2);
            if cp2.is_some() {
                cp2.map(|s| *s = *s + count);
            }else {
                new_pair_counts.insert(p2, *count);
            }
        }
        pair_counts = new_pair_counts;
    }

    pair_counts
}

fn compute_char_occurence(pair_counts: &HashMap<(char,char), u64>, char_list: &Vec<char>, last_char: char) -> (u64, u64) { // (min, max)
    let mut map = HashMap::new();
    for c in char_list {
        map.insert(c, 0);
    }
    for ((c1,_), count) in pair_counts {
        map.get_mut(c1).map(|c| *c=*c+count);
    }
    map.get_mut(&last_char).map(|c| *c=*c+1);
    (*map.values().min().unwrap(), *map.values().max().unwrap())
}


/// Test Input
/// Solutions: Part1 => 1588, Part2 => 2188189693529
fn test_input() -> String {
    String::from("NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C")
}

fn parsing(input: &str) -> (String, RuleSet) {
    let mut parts = input.split("\n\n");
    let template = parts.next().unwrap();
    let rules = parts.next().unwrap();

    let mut ruleset = HashMap::new();
    for rule in rules.lines() {
        let mut s = rule.split(" -> ");
        let mut pair = s.next().unwrap().chars();
        let c = s.next().unwrap().chars().next().unwrap();
        let pair = (pair.next().unwrap(), pair.next().unwrap());
        ruleset.insert(pair, c);
    }
    (String::from(template), ruleset)
}
