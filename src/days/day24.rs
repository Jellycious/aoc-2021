use crate::AOCDay;

use nom::{
    character,
    error,
    bytes::complete::take_till,
    sequence::preceded,
};


/*
 * Day 24: Arithmetic Logic Unit
 *
 * Try to figure out how MONAD works, and what is the highest and lowest number it will accept.
 * This has been solved by reverse engineering the input.
 */

pub struct Day24();

impl AOCDay for Day24 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 24 }
}

pub fn get() -> Day24 {Day24()}

/*
Below are observations and comments about the reverse-engineering of the input file.

### OBSERVATIONS
##### `z` acts as a stack
- The top of the stack is accessed by `mod z 26`
- The stack is increased by `mul z 26` (top of stack becomes 0)
- The stack is popped by `div z 26`

##### The code is separated in chunks 
- Every chunk consists of 18 instructions

##### Conditions
- A condition refers to the `eql` comparison on instruction 7 of a chunk (8 is simply negation of the result)

- If condition is **false** then z is multiplied by 26 (otherwise by 1)
This means that everytime we multiply z gets further from 0
Condition refers to the first eql, not the second eql (which invert 0 to 1 vice versa)

- If condition is **true** then `w+$v3` does not get put on top of the stack

##### 7 conditions are FALSE independent of inputs
- 7 time z is guaranteed to be multiplied by 26 (`mul z 26`)
- This implies that to force `z` to zero, **we must** divide `z` 7 times by 26!

##### Every chunk has three variables: `$v1, $v2, $v3` on instructions 5,6,16 respectively.
- `$v1` determines whether a variable gets popped from the stack
- `$v2` and `$v3` determine offsets for comparisons: (`[dx+$v3]+$v2 == w`)
- `$v3` is always positive
- if `$v2` > 9 then the condition is always **false** (because `$v3` is always positive)

##### If `div z 26` then the top value of the stack gets removed otherwise `div z 1`
- The stack gets popped 7 times (`div z 26`) (`div z $v1`)

### Solution:

##### Inspection of the input source code:
The following conditions must be true otherwize `z` will be non-zero:
1. d2==d3
2. d7+1==d6
3. d9==d8+2
4. d10==d5+3
5. d11+5==d4
6. d12+7==d1
7. d13==d0+6

These are constraints. We must find the highest and lowest value for which these constraints hold.

###### The solution for the maximum then is:
d0=3
d1=9
d2=9
d3=9
d4=9
d5=6
d6=9
d7=8
d8=7
d9=9
d10=9
d11=4
d12=2
d13=9
SOLUTION BY HAND: 39999698799429

###### The solution for the minimum is:
d0=1
d1=8
d2=1
d3=1
d4=6
d5=1
d6=2
d7=1
d8=1
d9=3
d10=4
d11=1
d12=1
d13=7
SOLUTION BY HAND: 18116121134117
 */


/// Chunk contains all neccesary information about a chunk (18 lines of instructions) in the input file
#[derive(Debug)]
struct Chunk {
    popped_stack: bool, // Variable 1
    eql_offset: i32, // Variable 2
    stack_offset: u32, // Variable 3
}

#[derive(Debug)]
struct Constraint(usize, i32, usize); // c.0+c.1 == c.2

/// Solve automatically
fn part1(input: &str) -> String {
    let chunks = parse(&input);
    let constraints = get_constraints(&chunks);
    solve(&constraints, false, 14)
}

fn part2(input: &str) -> String {
    let chunks = parse(&input);
    let constraints = get_constraints(&chunks);
    solve(&constraints, true, 14)
}

/// Derives constraints from the input
fn get_constraints(chunks: &Vec<Chunk>) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    let mut stack = vec![(999,999)];
    for i in 0..chunks.len() {
        let chunk = &chunks[i];
        let &(d, offset) = &stack[stack.len() - 1];
        if chunk.popped_stack {stack.pop();}
        if chunk.eql_offset > 9 { // condition will always be false and value is put on stack
            stack.push((i, chunk.stack_offset));
        }else { // assume that the condition must be true
            constraints.push(Constraint(d,offset as i32+chunk.eql_offset,i));
        }
    }
    constraints
}

/// Solves the constraints
fn solve(constraints: &Vec<Constraint>, min: bool, digit_length: usize) -> String {
    let mut digits = vec![0;digit_length];
    constraints.into_iter().for_each(|c| {
        let iter = (1..10).into_iter().filter(|n| n+c.1 > 0 && n+c.1 < 10);
        let left = if min {iter.min().unwrap()} else {iter.max().unwrap()};
        let right = left + c.1;
        digits[c.0] = left;
        digits[c.2] = right;
    });
    digits.into_iter().map(|d| d.to_string()).collect::<String>()
}


// --- PARSING ---
fn parse(input: &str) -> Vec<Chunk> {
    let lines: Vec<&str> = input.lines().collect();
    let mut chunks = Vec::new();
    let mut instr_parser = preceded::<&str,_,i32,error::Error<&str>,_,_>(take_till(|c: char| c.is_ascii_digit() || c == '-'), character::complete::i32);
    for i in 0..14 {
        let popped_stack = instr_parser(lines[4+18*i]).unwrap().1 == 26;
        let eql_offset = instr_parser(lines[5+18*i]).unwrap().1;
        let stack_offset = instr_parser(lines[15+18*i]).unwrap().1 as u32;
        chunks.push(Chunk{popped_stack,eql_offset,stack_offset});
    }
    chunks
}

