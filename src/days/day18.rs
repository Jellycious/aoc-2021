use crate::AOCDay;

use itertools::Itertools;

/*
 * Day 18: Snailfish
 *
 * Add snailfish numbers, which obey special rules.
 */

pub struct Day18();

impl AOCDay for Day18 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 18 }
}

pub fn get() -> Day18 {Day18()}

#[derive(Debug, Clone)]
enum Snail {
    LB,
    RB,
    Num(u32),
}

type Number = Vec<Snail>;

#[derive(Debug)]
enum Reduction {
    Explode(usize),
    Split(usize),
    Nothing,
}

fn part1(input: &str) -> String {
    let numbers = parse(&input);
    let mut iter = numbers.into_iter();
    let mut num = iter.next().unwrap();
    while let Some(mut snail) = iter.next() {
        addition(&mut num, &mut snail);
        reduce(&mut num);
    }
    magnitude(&num).to_string()
}

fn part2(input: &str) -> String {
    let numbers = parse(&input);
    let perms = numbers.into_iter().permutations(2);
    let mut max = 0;
    for perm in perms {
        let mut perm_iter = perm.into_iter();
        let mut n1 = perm_iter.next().unwrap();
        let mut n2 = perm_iter.next().unwrap();
        addition(&mut n1, &mut n2);
        reduce(&mut n1);
        max = u32::max(max, magnitude(&n1));
    }
    max.to_string()
}

fn magnitude(num: &Number) -> u32 {
    let mut stack = Vec::new();
    let mut iter = num.into_iter();
    while let Some(s) = iter.next() {
        match s {
            Snail::LB => {
                // Insert barrier
                stack.push(None);
            },
            Snail::RB => {
                let v2 = stack.pop().unwrap().unwrap();
                let v1 = stack.pop().unwrap().unwrap();
                stack.pop(); // remove barrier
                stack.push(Some(3*v1 + 2*v2));
            },
            Snail::Num(n) => {
                // check top of stack
                stack.push(Some(*n));
            },
        }
    }
    stack.pop().unwrap().unwrap()
}

fn reduce(num: &mut Number) {
    loop {
        match reduce_check(num) {
            Reduction::Explode(i) => {
                explode(num, i);
            },
            Reduction::Split(i) => {
                split(num, i);
            },
            Reduction::Nothing => {break;},
        }
    }
}

fn reduce_check(num: &Number) -> Reduction {
    // EXPLODES OCCUR BEFORE SPLITS
    let mut iter = num.iter().enumerate();
    let mut depth = 0;
    // Test for explosion
    while let Some((i, snail)) = iter.next() {
        match snail {
            Snail::LB => {
                if depth >= 4 {
                    return Reduction::Explode(i); // element at i explodes
                }
                depth += 1;
            },
            Snail::RB => {
                depth -= 1;
            },
            _ => {},
        }
    }
    // Test for split
    let mut iter = num.iter().enumerate();
    while let Some((i, snail)) = iter.next() {
        match snail {
            Snail::Num(n) => {if *n >= 10 {return Reduction::Split(i)}},
            _ => {},
        }
    }
    return Reduction::Nothing;
}

fn explode(snail: &mut Number, index: usize) {
    // pair at index explodes (index contains '[')
    let ln = inner_num(&snail[index+1]);
    let rn = inner_num(&snail[index+2]);
    // Increment the left and right side with appropriate values
    let mut left_side = snail[0..index].iter_mut().rev();
    while let Some(snail) = left_side.next() {
        match snail {
            Snail::Num(n) => {*n += ln;break;},
            _ => {},
        }
    }
    let mut right_side = snail[(index+3)..].iter_mut();
    while let Some(snail) = right_side.next() {
        match snail {
            Snail::Num(n) => {*n += rn;break;},
            _ => {},
        }
    }
    // Remove pair at index
    for _ in 0..4 {
        snail.remove(index); // O(n)
    }
    snail.insert(index, Snail::Num(0))
}

fn split(snail: &mut Number, index: usize) {
    // number at index splits into two numbers
    let n = inner_num(&snail[index]) as f32;
    let n1 = (n / 2.0).floor() as u32;
    let n2 = (n / 2.0).ceil() as u32;
    snail[index] = Snail::LB;
    snail.insert(index+1,Snail::RB);
    snail.insert(index+1,Snail::Num(n2));
    snail.insert(index+1,Snail::Num(n1));
}

fn inner_num(snail: &Snail) -> u32 {
    match snail {
        Snail::Num(n) => *n,
        _ => {panic!("Must be of num type")},
    }
}

/// Adds s2 to s1, leaving s2 empty
fn addition(s1: &mut Number, s2: &mut Number) {
    s1.insert(0, Snail::LB);
    s1.append(s2);
    s1.push(Snail::RB);
}

fn parse_line(input: &str) -> Number {
    let mut chars = input.trim().chars();
    let mut snails = Vec::new();
    while let Some(c) = chars.next() {
        match c {
            '[' => {snails.push(Snail::LB)},
            ']' => {snails.push(Snail::RB)},
            ',' => {},
            _ => {snails.push(Snail::Num(u32::from_str_radix(&c.to_string(), 10).unwrap()))},
        }
    }
    snails
}

fn parse(input: &str) -> Vec<Number> {
    let mut v = Vec::new();
    for line in input.lines() {
        let n = parse_line(line);
        v.push(n)
    }
    v
}

fn to_string(snail: &Number) -> String {
    let mut s = String::with_capacity(snail.len());
    let mut iter = snail.iter();
    while let Some(snail) = iter.next() {
        match snail {
            Snail::Num(n) => {s.push_str(&format!("({})", n));},
            Snail::RB => {s.push_str("]")},
            Snail::LB => {s.push_str("[")},
        }
    }
    s
}

fn test_input1() -> String {
    String::from("[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]")
}

fn test_explode1() -> String {
    String::from("[[[4,[[3,4],8]],5],5]")
}
fn test_explode2() -> String {
    String::from("[[[[[9,8],1],2],3],4]")
}
fn test_explode3() -> String {
    String::from("[7,[6,[5,[4,[3,2]]]]]")
}
fn test_explode4() -> String {
    String::from("[[6,[5,[4,[3,2]]]],1]")
}
fn test_explode5() -> String {
    String::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
}
fn test_explode6() -> String {
    String::from("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
}

fn test_input2() -> String {
    String::from("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]")
}

fn test_input3() -> String {
    String::from("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]")
}

fn test_bug1() -> String {
    String::from("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]")
}

fn test_bug2() -> String {
    String::from("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]")
}

fn test_magnitude1() -> String {
    String::from("[[1,2],[[3,4],5]]")
}
fn test_magnitude2() -> String {
    String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
}
fn test_magnitude3() -> String {
    String::from("[[[[1,1],[2,2]],[3,3]],[4,4]]")
}
