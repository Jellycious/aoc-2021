use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day9();

type Grid = Vec<Vec<u32>>;
type Coord = (usize, usize);

impl AOCDay for Day9 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 9 }
}

pub fn get() -> Day9 {Day9()}

fn part1(input: &str) -> String {
    let grid = parser(&input);
    let lows = low_points(&grid);
    let risk: u32 = lows.into_iter().map(|(x,y)| grid[y][x]+1).sum();
    String::from(format!("{}", risk))
}

fn part2(input: &str) -> String {
    let grid = parser(&input);
    let lows = low_points(&grid);
    // get sizes of various basins
    let mut sizes = Vec::new();
    for coord in lows {
        let s = basin_size(&grid, coord);
        sizes.push(s);
    }

    sizes.sort(); 
    let sum: u32 = sizes.into_iter().rev().take(3).product();
    String::from(format!("{}", sum))
}

fn basin_size(grid: &Grid, low_point: Coord) -> u32 {
    let mut visited = Vec::new();
    for _ in 0..grid.len() {
        let mut r = Vec::new();
        for _ in 0..grid[0].len() {
            r.push(false);
        }
        visited.push(r);
    }
    1 + _basin_size_visit_coord(grid, &mut visited, low_point)
}

fn _basin_size_visit_coord(grid: &Grid, mut visited: &mut Vec<Vec<bool>>, (x, y): Coord) -> u32 {
    let n = grid[y][x];

    let mut size = 0;
    let nc = get_neighbour_coords(x, y, &grid);

    for (x2, y2) in nc {
        if grid[y2][x2] < 9 {
            // potentially new node
            if !visited[y2][x2] && grid[y2][x2] > n { // visit node only if its bigger and not visited already
                visited[y2][x2] = true;
                // +1, because we have found another node, which flows into the basin
                size = size + 1 + _basin_size_visit_coord(&grid, &mut visited, (x2, y2));
            }
        }
    }
    size
}

fn low_points(grid: &Grid) -> Vec<Coord> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut lows = Vec::new();
    for y in 0..num_rows {
        for x in 0.. num_cols {
            let neighbours = get_neighbours(x,y,&grid);
            let e = grid[y][x];
            if !neighbours.iter().any(|n| n <= &e) {
                lows.push((x,y));
            }
        }
    }
    lows
}

fn test_input() -> String {
    String::from("2199943210
3987894921
9856789892
8767896789
9899965678")
}

fn get_neighbours(x: usize, y: usize, grid: &Grid) -> Vec<u32> {
    let mut v = Vec::new();
    if x > 0 {v.push(grid[y][x-1]);}
    if x < (grid[0].len() - 1) {v.push(grid[y][x+1]);}
    if y > 0 {v.push(grid[y-1][x]);}
    if y < (grid.len() - 1) {v.push(grid[y+1][x]);}
    v
}

fn get_neighbour_coords(x: usize, y: usize, grid: &Grid) -> Vec<Coord> {
    let mut coords = Vec::new();
    if x > 0 {coords.push((x-1, y))};
    if x < (grid[0].len() - 1) {coords.push((x+1, y))};
    if y > 0 {coords.push((x, y-1))};
    if y < (grid.len() - 1) {coords.push((x, y+1))};
    coords
}

fn parser(input: &str) -> Grid {
    let mut grid = Vec::new();
    let lines = input.lines();
    for l in lines {
        let nums = l.chars().map(|s|  s.to_digit(10).unwrap()).collect();
        grid.push(nums);
    }
    grid
}
