use crate::AOCDay;

use std::mem;

/*
 * Day 20: Trench Map
 *
 * Use image enhancement algorithm to create an image.
 */

pub struct Day20();

impl AOCDay for Day20 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 20 }
}

pub fn get() -> Day20 {Day20()}

type Image = Vec<Vec<bool>>;
type EAlg = Vec<bool>;

fn part1(input: &str) -> String {
    let (mut img, alg) = parse(&input);
    iterate(&mut img, &alg, 2);
    let lit = img.into_iter().flatten().filter(|c| *c == true);
    format!("{}", lit.count())
}

fn part2(input: &str) -> String {
    let (mut img, alg) = parse(&input);
    iterate(&mut img, &alg, 50);
    let lit = img.into_iter().flatten().filter(|c| *c == true);
    format!("{}", lit.count())
}

fn iterate(img: &mut Image, alg: &EAlg, steps: u32) {
    let mut inf_tile = false; // models tile color at infinity
    add_padding(img, inf_tile);

    let mut buffer1 = img.clone();
    let mut buffer2 = img.clone();
    for _ in 0..steps {
        add_padding(&mut buffer1, inf_tile); // increase both buffers
        add_padding(&mut buffer2, inf_tile); 
        for iy in 1..buffer2.len()-1 {
            for ix in 1..buffer2.len()-1 {
                let w = get_window(ix,iy,&mut buffer1);
                buffer2[iy][ix] = alg[w as usize]; 
            }
        }
        // update infinity tile
        let w = if inf_tile {0b0111111111} else {0x01};
        inf_tile = alg[w];
        update_border(&mut buffer2, inf_tile);
        // swap buffers
        mem::swap(&mut buffer1, &mut buffer2);     
    }
    mem::swap(img, &mut buffer1);
}

fn get_window(x: usize, y: usize, img: &Image) -> u32 {
    let mut val = 0;
    for iy in y-1..y+2 {
        for ix in x-1..x+2 {
            if img[iy][ix] {
                val = (val << 1) | 0x01;
            }else {
                val = val << 1;
            }
        }
    }
    val
}

fn add_padding(img: &mut Image, dark: bool) {
    for row in img.iter_mut() {
        row.insert(0, dark);
        row.insert(0, dark);
        row.append(&mut vec![dark, dark]);
    }
    let n = vec![dark; img.len()+4];
    img.insert(0, n.clone());
    img.insert(0, n.clone());
    img.push(n.clone());
    img.push(n)
}

fn update_border(img: &mut Image, dark: bool) {
    let z = img.len();
    for i in 0..z {
        img[i][0] = dark;
        img[i][z-1] = dark;
        img[0][i] = dark;
        img[z-1][i] = dark;
    }
}

fn img_to_string(img: &Image) -> String {
    let mut s = String::with_capacity(img.len() * (img.len()+1));
    let z = img.len();
    for y in 0..z {
        for x in 0..z {
            if img[y][x] {s.push('#')} else {s.push('.')}
        }
        s.push('\n');
    }
    s
}

fn parse(input: &str) -> (Image, EAlg) {
    let mut parts = input.split("\n\n");
    let alg_s = parts.next().unwrap();
    let mut enhancement_alg = Vec::new();
    for c in alg_s.chars() {
        match c{
            '.' => {enhancement_alg.push(false)},
            '#' => {enhancement_alg.push(true)},
            _ => {},
        }
    }
    let img_s = parts.next().unwrap();
    let mut img = Vec::new();
    for line in img_s.lines() {
        let mut r = Vec::new();
        for c in line.trim_end().chars() {
            match c{
                '.' => {r.push(false)},
                '#' => {r.push(true)},
                _ => {},
            }
        }
        img.push(r);
    }
    (img, enhancement_alg)
}

fn test_input() -> String {
    String::from("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###")
}
