use crate::AOCDay;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/*
 * Day 15: Chiton
 * Path finding algorithm, Find shortest path.
 *
 * Optimization: use A* instead of Dijkstra
 */

pub struct Day15();

impl AOCDay for Day15 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 15 }
}
pub fn get() -> Day15 {Day15()}

type Grid = Vec<Vec<u32>>;
type Coord = (usize, usize);

#[derive(Eq,PartialEq)]
struct Node(Coord, u32);

// Create reversed Ord implementation for use with BinaryHeap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> String {
    let grid = parser(&input);
    let cost = shortest_path(&grid);
    format!("{}", cost)
}

fn part2(input: &str) -> String {
    let grid = parser(&input);
    let grid = expand_map(&grid);
    let cost = shortest_path(&grid);
    format!("{}", cost)

}

fn shortest_path(grid: &Grid) -> u32 { // (Path, Cost)
    // Dijkstra Algorithm
    let xlen = grid[0].len();
    let ylen = grid.len();
    let goal = (xlen-1, ylen-1);
    
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    let mut dist: Vec<Vec<u32>> = (0..ylen).map(|_| (0..xlen).map(|_| u32::MAX).collect()).collect();

    dist[0][0] = 0;
    queue.push(Node((0,0), 0));

    while let Some(Node((x,y),c)) = queue.pop() {
        if (x,y) == goal { return c; }
        if c > dist[y][x] { continue; }

        let ns = get_adjacent_inidces(x,y,grid);
        for (x2,y2) in ns {
            let node = Node((x2,y2), dist[y][x] + grid[y2][x2]); // node should contain coordinate and distance to it
            if node.1 < dist[y2][x2] { // found better path
                dist[y2][x2] = node.1;
                queue.push(node);
            }
        }
    }
    0
}

fn expand_map(grid: &Grid) -> Grid {
    let mut new_grid = Vec::new();
    for yi in 0..5 {
        for y in 0..grid.len() {
            let mut row = Vec::new();
            for xi in 0..5 {
                for x in 0..grid[0].len() {
                    row.push(wrap(grid[y][x]+yi+xi));
                }
            }
            new_grid.push(row);
        }
    }
    new_grid
}

fn wrap(n: u32) -> u32 { // safe, because a number can't reach 18
    if n == 9 {9}
    else {n % 9}
}



/// Returns a list of indices of the neighbours of grid[y][x].
fn get_adjacent_inidces(x: usize, y: usize, grid: &Grid) -> Vec<(usize, usize)> {
    let (row_size, col_size) = (grid.len() as i32, grid[0].len() as i32);
    let (x, y) = (x as i32, y as i32);
    let indices: Vec<(i32, i32)> = vec![(x-1,y), (x,y-1),(x,y+1),(x+1,y)];
    indices.into_iter().filter(|(x,y)| *x >= 0 && *x < col_size && *y >= 0 && *y < row_size).map(|(x,y)| (x as usize, y as usize)).collect()
}

fn test_input() -> String {
    String::from("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581")
}

fn parser(input: &str) -> Grid {
        input.lines().map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        }).collect()
}
