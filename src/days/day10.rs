use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day10();

impl AOCDay for Day10 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 10 }
}

pub fn get() -> Day10 {Day10()}

fn part1(input: &str) -> String {
    let score: u32 = input.lines().map(syntax_err_score).sum();
    String::from(format!("{}", score))
}

fn part2(input: &str) -> String {
    let lines = input.lines().filter(|l| syntax_err_score(l) == 0); // complete and incomplete lines
    let mut scores: Vec<u64> = lines.map(|s| completion_score(&complete_syntax(s))).collect();
    scores.sort();
    String::from(format!("{}", scores[scores.len() / 2]))
}

fn completion_score(line: &str) -> u64 {
    let mut score = 0;
    for c in line.chars() {
        match c {
            ')' => score = (score * 5) + 1,
            ']' => score = (score * 5) + 2,
            '}' => score = (score * 5) + 3,
            '>' => score = (score * 5) + 4,
            _ => panic!("Unexpected char: {}", c)
        }
    }
    score
}

fn complete_syntax(line: &str) -> String {
    let chars = line.trim().chars();
    let mut stack: Vec<char> = Vec::new();
    for c in chars {
        match c {
            '[' => {stack.push(']')},
            '{' => {stack.push('}')},
            '(' => {stack.push(')')},
            '<' => {stack.push('>')},
            ')' => {stack.pop();},
            ']' => {stack.pop();},
            '}' => {stack.pop();},
            '>' => {stack.pop();},
            _ => {panic!("unrecognized char: {}", c);}
        }
    }
    stack.iter().rev().collect::<String>()
}


fn syntax_err_score(line: &str) -> u32 {
    let chars = line.trim().chars();
    let mut stack: Vec<char> = Vec::new();
    for c in chars {
        match c {
            '[' => {stack.push(c)},
            '{' => {stack.push(c)},
            '(' => {stack.push(c)},
            '<' => {stack.push(c)},
            ')' => {
                let s = stack.pop().unwrap();
                if s != '(' {return 3;}
            },
            ']' => {
                let s = stack.pop().unwrap();
                if s != '[' {return 57;}
            },
            '}' => {
                let s = stack.pop().unwrap();
                if s != '{' {return 1197;}
            },
            '>' => {
                let s = stack.pop().unwrap();
                if s != '<' {return 25137;}
            },
            _ => {panic!("unrecognized char: {}", c);}
        }
    }
    0
}

fn test_input() -> String {
    String::from("[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]")
}
