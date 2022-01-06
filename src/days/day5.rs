use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day5();

type Line = ((u32,u32), (u32,u32));

impl AOCDay for Day5 {
    fn part1(&self, input: &str) -> Option<String> { Some(number_of_overlapping(input, false)) }
    fn part2(&self, input: &str) -> Option<String> { Some(number_of_overlapping(input, true)) }
    fn get_num(&self) -> u32 { 5 }
}

pub fn get() -> Day5 {Day5()}

fn number_of_overlapping(input: &str, diagonal: bool) -> String {
    let lines = parsing::parse(&input);
    let vents = grid_with_vents(&lines, diagonal);

    let mut overlapped = 0;
    for r in &vents {
        for n in r {
            if *n > 1 {
                overlapped+=1;
            }
        }
    }
    String::from(format!("{}", overlapped))
}

fn grid_with_vents(lines: &Vec<Line>, diagonal: bool) -> Vec<Vec<u32>> {
    let max_coord = max_coord(&lines);

    let mut grid: Vec<Vec<u32>> = Vec::with_capacity((max_coord+1) as usize); // construct grid of zeroes
    for _ in 0..max_coord+1 {
        let mut v = Vec::with_capacity((max_coord+1) as usize);
        for _ in 0..(max_coord as usize)+1 {
            v.push(0);
        }
        grid.push(v);
    }

    for l in lines { // mark the lines
        if diagonal {
            mark_line(&mut grid, &l);
        }else {
            mark_line_horizontal_vertical(&mut grid, &l);
        }
    }
    grid
}

fn mark_line_horizontal_vertical(grid: &mut Vec<Vec<u32>>, line: &Line) {
    let ((x1,y1),(x2,y2)) = line;
    let yd = *y2 as i32 - *y1 as i32;
    let xd = *x2 as i32 - *x1 as i32;
    if i32::abs(yd) > 0 && i32::abs(xd) == 0 {
        // vertical line (so x1==x2)
        let yd = yd / i32::abs(yd);
        let mut i = *y1 as i32;
        loop {
            grid[i as usize][*x1 as usize] += 1;
            if i == *y2 as i32 {break;} 
            i += yd;
        }
    }else if i32::abs(xd) > 0 && i32::abs(yd) == 0 {
        // horizontal line
        let xd = xd / i32::abs(xd);
        let mut i = *x1 as i32;
        loop {
            grid[*y1 as usize][i as usize] += 1;
            if i == *x2 as i32 {break;} 
            i += xd;
        }
    }
}

fn mark_line(grid: &mut Vec<Vec<u32>>, line: &Line) {
    let ((x1,y1),(x2,y2)) = line;
    let yd = *y2 as i32 - *y1 as i32;
    let xd = *x2 as i32 - *x1 as i32;

    let xd_n = if xd != 0 {xd / i32::abs(xd)} else {0};
    let yd_n = if yd != 0 {yd / i32::abs(yd)} else {0};

    let (mut x,mut y): (i32, i32) = (*x1 as i32, *y1 as i32);
    loop {
        grid[y as usize][x as usize] += 1;
        if y as u32 == *y2 && x as u32 == *x2 {break;}
        x += xd_n;
        y += yd_n;
    }
}

fn max_coord(lines: &Vec<Line>) -> u32 {
    let mut max = 0;
    for l in lines {
        max = u32::max(max, l.0.0);
        max = u32::max(max, l.0.1);
        max = u32::max(max, l.1.0);
        max = u32::max(max, l.1.1);
    }
    max 
}

fn test_input() -> String {
    String::from("0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2")
}

mod parsing {
    use nom;
    use nom::IResult;
    use nom::bytes::complete::tag;
    use nom::character::complete::digit0;

    use super::Line;

    fn parse_line(input: &str) -> Line {
        let coord1 = nom::sequence::separated_pair(digit0, tag(","), digit0);
        let coord2 = nom::sequence::separated_pair(digit0, tag(","), digit0);
        let mut coords = nom::sequence::separated_pair(coord1, tag(" -> "), coord2);
        let res: IResult<&str, ((&str,&str),(&str,&str))> = coords(input);
        res.map(|(_, ((x1,y1),(x2,y2)))| ((x1.parse().unwrap(),y1.parse().unwrap()),(x2.parse().unwrap(),y2.parse().unwrap()))).unwrap()
    }
    
    pub fn parse(input: &str) -> Vec<Line> {
        let lines = input.lines();
        let mut v = Vec::new();
        for l in lines {
            v.push(parse_line(l));
        }
        v
    }

}
