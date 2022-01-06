use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day2();

impl AOCDay for Day2 {
    fn part1(&self, input: &str) -> Option<String> { Some(calculate_position(input)) }
    fn part2(&self, input: &str) -> Option<String> { Some(calculate_position2(input)) }
    fn get_num(&self) -> u32 { 2 }
}

pub fn get() -> Day2 {Day2()}

fn calculate_position(input: &str) -> String {
    //TODO: PARSE INPUT
    let dirs = parsing::parse(input);
    let mut x = 0;
    let mut d = 0;
    for dir in dirs {
        match dir {
            parsing::Dir::Up(n) => {d -= n;},
            parsing::Dir::Down(n) => {d += n;},
            parsing::Dir::Forward(n) => {x += n},
        }
    }
    String::from(format!("{}", x * d))
}

fn calculate_position2(input: &str) -> String {
    let dirs = parsing::parse(input);
    let mut x = 0;
    let mut d = 0;
    let mut aim = 0;
    for dir in dirs {
        match dir {
            parsing::Dir::Up(n) => {aim -= n;},
            parsing::Dir::Down(n) => {aim += n;},
            parsing::Dir::Forward(n) => {x += n; d += n * aim},
        }
    }
    String::from(format!("{}", x * d))
}

mod parsing {
    use nom::bytes::complete::tag;
    use nom::{IResult};
    use nom::combinator::{recognize};
    use nom::branch::{alt};
    use nom::sequence::{separated_pair};
    use nom::error::Error;
    use nom::character;

    #[derive(Debug)]
    pub enum Dir {
        Up(u32),
        Down(u32),
        Forward(u32)
    }

    pub fn parse(input: &str) -> Vec<Dir> {
        let lines = input.lines();
        let mut v = Vec::new();
        for line in lines {
            let l = parse_line(line);
            if l.is_ok() {
                v.push(l.unwrap().1);
            }else {
                println!("{}", l.unwrap_err());
                panic!("PARSE ERROR");
            }
        }
        v
    }

    fn parse_line(input: &str) -> IResult<&str, Dir> {
        let space = character::complete::char::<_, (&str, _)>(' ');
        let foward = recognize(tag("forward"));
        let down = recognize(tag("down"));
        let up = recognize(tag("up"));
        let dir = alt((foward, up, down));
        let number = character::complete::u32;

        let mut parser= separated_pair(dir, space, number);
        let res = parser(input);
        let result = res.map_err(|e| {// we have to convert to the default error type
            e.map(|err| {
                Error {input: err.0, code: err.1}
            })
        });
        result.map(|(rem, (d, num))| {
            match d {
                "forward" => (rem, Dir::Forward(num)),
                "down" => (rem, Dir::Down(num)),
                "up" => (rem, Dir::Up(num)),
                _ => panic!("PARSE ERROR"),
            }
        })
    }

    #[cfg(test)]
    pub mod test {
        use super::*;

        #[test]
        fn test_parser() {
            let res = parse_line("forward 8");
            println!("{:?}", res);
        }
    }
}
