use crate::AOCDay;

use std::collections::HashSet;

use nom::IResult;
use nom::sequence::separated_pair;
use nom::multi::separated_list0;
use nom::character::complete::alpha1;
use nom::bytes::complete::tag;

/*
 * Template for a implementing a day
 */

pub struct Day8();

impl AOCDay for Day8 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 8 }
}

// Maps a signal to its corresponding true signal
type Mapping = (char, char);

fn part1(input: &str) -> String {
    let parsed = parser(&input);
    let mut count = 0;
    for (_, os) in parsed {
        for o in os {
            match o.len() {
                2 => count+=1,
                3 => count+=1,
                4 => count+=1,
                7 => count+=1,
                _ => {},
            }
        }
    }
    String::from(format!("{}", count))
}

fn part2(input: &str) -> String {
    let parsed = parser(&input);
    let sequences = parsed.into_iter().map(|(ss, os)| {
        // convert to HashSet
        let hs: Vec<HashSet<char>> = ss.into_iter().map(|s| s.chars().collect()).collect();
        let ho: Vec<HashSet<char>> = os.into_iter().map(|s| s.chars().collect()).collect();
        (hs, ho)
    });

    let mut total = 0;
    for (seq, out) in sequences {
        let result = decrypt_output(seq, out);
        total += result;
    }
    String::from(format!("{}", total))
}

fn decrypt_output(seq: Vec<HashSet<char>>, output: Vec<HashSet<char>>) -> u32 {
    let mut lfives: Vec<HashSet<char>> = Vec::with_capacity(3);
    let mut lsixes: Vec<HashSet<char>> = Vec::with_capacity(3);

    let mut one: Option<HashSet<char>> = None;
    let mut seven: Option<HashSet<char>> = None;
    let mut eight: Option<HashSet<char>> = None;
    let mut four: Option<HashSet<char>> = None;

    // Separate segment sequences by number of segments
    for hs in seq {
        match hs.len() {
            2 => {one = Some(hs)},
            3 => {seven = Some(hs)},
            4 => {four = Some(hs)},
            7 => {eight = Some(hs)},
            5 => {lfives.push(hs)},
            6 => {lsixes.push(hs)},
            _ => panic!("Encountered Uknown Length Segment Sequence {:?}", hs),
        }
    }
    let one = one.unwrap();
    let seven = seven.unwrap();
    let eight = eight.unwrap();
    let four = four.unwrap();
    
    let nine = determine_nine(&mut lsixes, &four);
    let e = (*eight.difference(&nine).next().unwrap(), 'e');
    let (zero, six) = determine_zero_six(lsixes, &one);

    let three = determine_three(&mut lfives); 

    let mut three_clone = three.clone(); // determine b segment
    three_clone.insert(e.0);
    let b = (*eight.difference(&three_clone).next().unwrap(), 'b');

    let (two, five) = determine_two_and_five(lfives, &b); // use b segment to identify 2 and 5

    // Determine score
    let mut output_value = 0;
    for s in output {
        let val: u32;
        if zero.eq(&s) {
            val = 0;
        }else if one.eq(&s) {
            val = 1;
        }else if two.eq(&s) {
            val = 2;
        }else if three.eq(&s) {
            val = 3;
        }else if four.eq(&s) {
            val = 4;
        }else if five.eq(&s) {
            val = 5;
        }else if six.eq(&s) {
            val = 6;
        }else if seven.eq(&s) {
            val = 7;
        }else if eight.eq(&s) {
            val = 8;
        }else { val = 9}
        output_value = output_value * 10 + val;
    }
    output_value
}

fn determine_three(fives: &mut Vec<HashSet<char>>) -> HashSet<char> {
    assert!(fives.len() == 3);
    let s1 = &fives[0];
    let s2 = &fives[1];
    let s3 = &fives[2];
    if s1.intersection(s2).count() == 3 {
        return fives.remove(2);// s3 is three
    }else if s1.intersection(s3).count() == 3 {
        return fives.remove(1);
    }else if s2.intersection(s3).count() == 3 {
        return fives.remove(0);
    }else {
        panic!("Could not determine three");
    }
}

fn determine_nine(sixes: &mut Vec<HashSet<char>>, four: &HashSet<char>) -> HashSet<char> {
    assert!(sixes.len() == 3);
    for i in 0..3 {
        if four.intersection(&sixes[i]).count() == 4 {
            return sixes.remove(i);
        }
    }
    panic!("Could not determine nine");
}

// Requires that sixes only contain zero and six.
fn determine_zero_six(mut sixes: Vec<HashSet<char>>, one: &HashSet<char>) -> (HashSet<char>, HashSet<char>) {
    assert!(sixes.len() == 2);
    let s1 = sixes.pop().unwrap();
    let s2 = sixes.pop().unwrap(); 
    if one.is_subset(&s1) {(s1, s2)}else {(s2, s1)}
}

fn determine_two_and_five(mut fives: Vec<HashSet<char>>, b: &Mapping) -> (HashSet<char>, HashSet<char>) {
    assert!(fives.len() == 2);
    let s1 = fives.pop().unwrap();
    let s2 = fives.pop().unwrap();
    if s1.contains(&b.0) {(s2, s1)} else if s2.contains(&b.0) {(s1, s2)} else {panic!("Could not determine two and five");}
}


fn test_input() -> String {
    let s = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    String::from(s)
}

fn test_input2() -> String {
    String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe")
}

pub fn get() -> Day8 {Day8()}


fn parser(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input.lines().map(|l| parse_line(l.trim())).collect()
}

fn parse_line(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut pair = separated_pair(parse_list_of_chars, tag(" | "), parse_list_of_chars);
    let res: IResult<&str, (Vec<&str>, Vec<&str>)> = pair(input);
    res.unwrap().1
}

fn parse_list_of_chars(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(nom::character::complete::char(' '), alpha1)(input)
}
