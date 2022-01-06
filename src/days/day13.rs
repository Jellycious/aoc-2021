use crate::AOCDay;
use std::collections::HashSet;

/*
 * Day 12
 * Folding Transparent Paper
 * 
 * Optimization:
 *  Keep solely track of the dot coordinates and try to map folds onto other coordinates
 */

pub struct Day13();

impl AOCDay for Day13 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 13 }
}

#[derive(Debug)]
struct Paper {
    dots: Dots,
    xlen: usize,
    ylen: usize
}

impl ToString for Paper {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity((self.xlen + 1) * self.ylen);
        for y in 0..self.ylen {
            for x in 0..self.xlen {
                if self.dots.contains(&(x,y)) {
                    s.push('#');
                }else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        return s;
    }
}

type Dots = HashSet<(usize, usize)>;

#[derive(Debug)]
enum Fold {
    Y(usize),
    X(usize),
}

pub fn get() -> Day13 {Day13()}

fn part1(input: &str) -> String {
    let (mut paper, folds) = parsing(&input);
    fold_paper(&mut paper, &folds[0]);
    String::from(format!("{}", paper.dots.len()))
}

fn part2(input: &str) -> String {
    let (mut paper, folds) = parsing(&input);
    for f in folds {
        fold_paper(&mut paper, &f);
    }
    String::from(format!("\n{}", paper.to_string()))
}

fn fold_paper(paper: &mut Paper, fold: &Fold) {
    match fold {
        Fold::Y(n) => {fold_y(paper, *n);}
        Fold::X(n) => {fold_x(paper, *n);}
    }
}

fn fold_x(paper: &mut Paper, f: usize) {
    let mut min = 0; // possibly need to shift all dots afterwards
    let mut new_coords: Vec<(i32, i32)> = Vec::new();
    for (x, y) in paper.dots.iter() {
        if *x > f {
            let x_new = 2 * f as i32 - *x as i32;
            min = i32::min(min, x_new);
            new_coords.push((x_new, *y as i32));
        } else if *x < f {
            new_coords.push((*x as i32,*y as i32));
        }
    }
    let shift = i32::abs(min);
    let mut new_dots: HashSet<(usize,usize)> = HashSet::new();  
    for (x,y) in new_coords.into_iter() {
        new_dots.insert(((x+shift) as usize, y as usize)); 
    }
    paper.dots = new_dots; // assign newly registered coordinates
    paper.xlen = usize::max(f, paper.xlen-1-f); // assign new xlen
}

fn fold_y(paper: &mut Paper, f: usize) {
    let mut min = 0; // possibly need to shift all dots afterwards
    let mut new_coords: Vec<(i32, i32)> = Vec::new();
    for (x, y) in paper.dots.iter() {
        if *y > f {
            let y_new = 2 * f as i32 - *y as i32;
            min = i32::min(min, y_new);
            new_coords.push((*x as i32, y_new));
        } else if *y < f {
            new_coords.push((*x as i32,*y as i32));
        }
    }
    let shift = i32::abs(min);
    let mut new_dots: HashSet<(usize,usize)> = HashSet::new();  
    for (x,y) in new_coords.into_iter() {
        new_dots.insert((x as usize, (y+shift) as usize)); 
    }
    paper.dots = new_dots; // assign newly registered coordinates
    paper.ylen = usize::max(f, paper.ylen-1-f); // assign new xlen
}

fn test_input1() -> String {
    String::from("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5")
}


fn parsing(input: &str) -> (Paper, Vec<Fold>) {
    let mut parts = input.split("\n\n");
    let coords = parts.next().unwrap();
    let folds = parts.next().unwrap();

    let mut dots: HashSet<(usize, usize)> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for coord in coords.lines() {
        let mut n = coord.split(',');
        let x: usize = n.next().unwrap().parse().unwrap();
        let y: usize = n.next().unwrap().parse().unwrap();
        max_x = usize::max(max_x, x);
        max_y = usize::max(max_y, y);
        dots.insert((x,y));
    }

    let mut fold_instrs = Vec::new();
    for fold in folds.lines() {
        let mut split = fold.split('=');
        let tag = split.next().unwrap();
        let num: usize = split.next().unwrap().parse().unwrap();
        if tag == "fold along y" {
            fold_instrs.push(Fold::Y(num));
        }else if tag == "fold along x"{
            fold_instrs.push(Fold::X(num));
        }else {
            panic!("Unrecognized tag: {}", tag);
        }
    }

    (Paper{dots, xlen: max_x+1, ylen: max_y+1}, fold_instrs)
}
