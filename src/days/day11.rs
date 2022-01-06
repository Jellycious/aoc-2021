use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day11();

type Grid = Vec<Vec<u32>>;

impl AOCDay for Day11 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 11 }
}

pub fn get() -> Day11 {Day11()}

fn part1(input: &str) -> String {
    let mut squids = parser(&input);
    let mut ftotal = 0;
    for _ in 0..100 {
        let f = simulate_day(&mut squids);
        ftotal += f;
    }
    String::from(format!("{}", ftotal))
}


fn part2(input: &str) -> String {
    let mut squids = parser(&input);
    let total_squids = squids.len() * squids[0].len();
    let mut days = 0;
    loop {
        days += 1;
        let f = simulate_day(&mut squids) as usize;
        if f == total_squids {
            break;
        }
    }
    String::from(format!("{}", days))
}

fn test_input() -> String {
    String::from("5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526")
}

fn simulate_day(grid: &mut Grid) -> u32 {
    let mut flashes = 0;
    for y in 0..grid.len() { // Step 1: increase all indices by 1
        for x in 0..grid[0].len() {
            grid[y][x] += 1;
        }
    }
    for y in 0..grid.len() { // Step 1: increase all indices by 1
        for x in 0..grid[0].len() {
            if grid[y][x] > 9 {
                flash(x, y, grid,&mut flashes);
            }
        }
    }
    flashes
}

/// Octopus at (x,y) flashes and might trigger neighbours
fn flash(x: usize, y: usize, grid: &mut Grid, flashes: &mut u32) {
    *flashes = *flashes + 1;
    grid[y][x] = 0;
    let ns = get_adjacent_inidces(x, y, &grid);
    for (x2, y2) in ns {
        let n = grid[y2][x2];
        if n >= 9 {
            flash(x2,y2,grid,flashes);
        } else if n > 0 {
            grid[y2][x2] += 1;
        }
    }
}

/// Returns a list of indices of the neighbours of grid[y][x].
fn get_adjacent_inidces(x: usize, y: usize, grid: &Grid) -> Vec<(usize, usize)> {
    let (row_size, col_size) = (grid.len() as i32, grid[0].len() as i32);
    let (x, y) = (x as i32, y as i32);
    let indices: Vec<(i32, i32)> = vec![(x-1,y-1), (x-1,y), (x-1,y+1),(x,y-1),(x,y+1),(x+1,y-1),(x+1,y),(x+1,y+1)];
    indices.into_iter().filter(|(x,y)| *x >= 0 && *x < col_size && *y >= 0 && *y < row_size).map(|(x,y)| (x as usize, y as usize)).collect()
}

fn parser(input: &str) -> Grid {
    input.lines().map(|l| l.chars().map(|s| s.to_digit(10).unwrap()).collect()).collect()
}
