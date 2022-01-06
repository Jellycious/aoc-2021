use crate::AOCDay;

/*
 * Template for a implementing a day
 */

pub struct Day4();

pub type Board = Vec<Vec<(u32, bool)>>;
pub type Draw = Vec<u32>;

impl AOCDay for Day4 {
    fn part1(&self, input: &str) -> Option<String> { Some(find_winning_board(input)) }
    fn part2(&self, input: &str) -> Option<String> { Some(find_losing_board(input)) }
    fn get_num(&self) -> u32 { 4 }
}

pub fn get() -> Day4 {Day4()}

fn find_winning_board(input: &str) -> String {
    let (draw, mut boards) = parsing::parse(&input);

    let mut winning_board: Option<Board> = None;
    let mut winning_draw: Option<u32> = None;

    'outer: for d in draw {
        for b in boards.iter_mut() {
            mark_num(b, d);

            if bingo(b) {
                winning_board = Some(b.clone());
                winning_draw = Some(d);
                break 'outer;
            }
        }
    }

    // find winning score for board
    let board = winning_board.unwrap();
    let draw = winning_draw.unwrap();
    let score = compute_score(&board, draw);
    String::from(format!("{}", score))
}

fn find_losing_board(input: &str) -> String {
    let (draw, boards) = parsing::parse(&input);

    let mut losing_boards = boards;

    let mut draw_iter = draw.iter();
    // Pretty inefficient, could be improved significantly
    'outer: while let Some(n) = draw_iter.next() {
        let mut v: Vec<Board> = Vec::with_capacity(losing_boards.len());
        for mut b in losing_boards.into_iter() {
            mark_num(&mut b, *n);

            if !bingo(&b) {
                v.push(b);
            }
        }
        losing_boards = v;
        if losing_boards.len() == 1 {
            break 'outer; // found losing board
        }
    }

    let mut b = losing_boards.pop().unwrap();
    let mut winning_draw = 0;
    for d in draw_iter {
        mark_num(&mut b, *d);
        if bingo(&b) {
            winning_draw = *d;
            break;
        }
    }
    let score = compute_score(&b, winning_draw);
    String::from(format!("{}", score))
}

fn compute_score(board: &Board, draw: u32) -> u32 {
    let mut score = 0;
    for r in 0..board.len() {
        for c in 0..board[0].len() {
            if !board[r][c].1 { score += board[r][c].0; }
        }
    }
    score * draw 
}

fn bingo(board: &Board) -> bool {
    let c = marked_column(board, 0) || marked_column(board, 1) || marked_column(board, 2) || marked_column(board, 3) || marked_column(board, 4);
    let r = marked_row(board, 0) || marked_row(board, 1) || marked_row(board, 2) || marked_row(board, 3) || marked_row(board, 4);
    c || r
}

fn mark_num(board: &mut Board, num: u32) {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j].0 == num { board[i][j].1 = true };
        }
    }
}

fn marked_column(board: &Board, col: usize) -> bool {
    for i in 0..board.len() {
        if !board[i][col].1 {
            return false;
        }
    }
    return true;
}

fn marked_row(board: &Board, row: usize) -> bool {
    for i in 0..board[0].len() {
        if !board[row][i].1 {
            return false;
        }
    }
    return true;
}


fn get_test_input() -> String {
    let mut s = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n");
    s.push_str("\n");
    s.push_str("22 13 17 11  0\n");
    s.push_str(" 8  2 23  4 24\n");
    s.push_str("21  9 14 16  7\n");
    s.push_str(" 6 10  3 18  5\n");
    s.push_str(" 1 12 20 15 19\n");
    s.push_str("\n");
    s.push_str(" 3 15  0  2 22\n");
    s.push_str(" 9 18 13 17  5\n");
    s.push_str("19  8  7 25 23\n");
    s.push_str("20 11 10 24  4\n");
    s.push_str("14 21 16 12  6\n");
    s.push_str("\n");
    s.push_str("14 21 17 24  4\n");
    s.push_str("10 16 15  9 19\n");
    s.push_str("18  8 23 26 20\n");
    s.push_str("22 11 13  6  5\n");
    s.push_str(" 2  0 12  3  7\n");
    s
}

pub mod parsing {
    use super::{Board, Draw};

    fn parse_draw(draw: &str) -> Draw {
        let nums = draw.split(',');
        let mut v: Vec<u32> = Vec::new();
        for s in nums {
            let n: u32 = s.parse().unwrap();
            v.push(n);
        }
        v
    }

    fn parse_row(row: &str) -> Vec<(u32, bool)> {
        let nums = row.split_whitespace();
        let mut v = Vec::new();
        for n in nums {
            v.push((n.parse().unwrap(), false));
        }
        v
    }

    fn parse_board(board: &str) -> Vec<Vec<(u32, bool)>> {
        let rows: Vec<&str>  = board.split('\n').collect();
        let mut board = Vec::new();
        for r in rows {
            let row = parse_row(r);
            board.push(row);
        }
        board
    }

    pub fn parse(input: &str) -> (Draw, Vec<Board>) {
        let mut lines = input.lines();

        let draw = lines.next().unwrap();
        let draw = parse_draw(draw); 
        lines.next();

        let mut boards: Vec<Board> = Vec::new();
        let boards_s: Vec<&str> = lines.collect();

        for i in 0..(boards_s.len() / 6) {
            let mut s = String::new();
            s.push_str(format!("{}\n", boards_s[i*6]).as_str());
            s.push_str(format!("{}\n", boards_s[i*6+1]).as_str());
            s.push_str(format!("{}\n", boards_s[i*6+2]).as_str());
            s.push_str(format!("{}\n", boards_s[i*6+3]).as_str());
            s.push_str(format!("{}", boards_s[i*6+4]).as_str());
            boards.push(parse_board(&s));
        };
        (draw, boards)
    }

}
